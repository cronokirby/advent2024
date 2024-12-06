use anyhow::anyhow;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

enum WhichDays {
    All,
    Just(u32),
}

impl WhichDays {
    fn parse(arg: Option<&str>) -> anyhow::Result<Self> {
        match arg {
            None | Some("a") | Some("all") => Ok(Self::All),
            Some(x) => {
                let u = u32::from_str_radix(x, 10)?;
                Ok(Self::Just(u))
            }
        }
    }
}

struct Args {
    path: PathBuf,
    which_days: WhichDays,
}

impl Args {
    fn parse(args: &[String]) -> anyhow::Result<Self> {
        if args.len() < 2 {
            return Err(anyhow!(
                "expected args to have len >= 2, found: {}",
                args.len()
            ));
        }
        let path = PathBuf::from(&args[1]);
        let which_days = WhichDays::parse(args.get(2).map(|x| x.as_str()))?;
        return Ok(Self { path, which_days });
    }
}

fn is_dir(dir: &Path) -> anyhow::Result<()> {
    if !dir.is_dir() {
        return Err(anyhow!("expected {:?} to be a directory", dir));
    }
    Ok(())
}

fn is_file(file: &Path) -> anyhow::Result<()> {
    if !file.is_file() {
        return Err(anyhow!("expected {:?} to be a file", file));
    }
    Ok(())
}

#[derive(Debug)]
struct TestFile {
    number: u32,
    input: PathBuf,
    output: PathBuf,
}

impl TestFile {
    fn extract(dir: &Path) -> anyhow::Result<Self> {
        let number = dir
            .components()
            .last()
            .expect("should have at least one component")
            .as_os_str()
            .to_string_lossy();
        let number = u32::from_str_radix(&number, 10)?;
        let input = dir.join("in.txt");
        is_file(&input)?;
        let output = dir.join("out.txt");
        is_file(&output)?;
        Ok(Self {
            number,
            input,
            output,
        })
    }
}

#[derive(Debug)]
struct PartFiles {
    input: Option<PathBuf>,
    tests: Vec<TestFile>,
}

impl PartFiles {
    fn extract(dir: &Path) -> anyhow::Result<Self> {
        is_dir(dir)?;
        let input = dir.join("in.txt");
        let input = if is_file(&input).is_ok() {
            Some(input)
        } else {
            None
        };
        let test_dir = dir.join("tests");
        let mut tests = Vec::new();
        for entry in fs::read_dir(test_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                tests.push(TestFile::extract(&path)?);
            }
        }
        Ok(Self { input, tests })
    }
}

#[derive(Debug)]
struct DayFiles {
    part_a: PartFiles,
    part_b: Option<PartFiles>,
}

impl DayFiles {
    fn extract(dir: &Path) -> anyhow::Result<Self> {
        let a_path = dir.join("A");
        let b_path = dir.join("B");
        Ok(Self {
            part_a: PartFiles::extract(&a_path)?,
            part_b: PartFiles::extract(&b_path).ok(),
        })
    }
}

#[derive(Debug)]
struct Files {
    days: BTreeMap<u32, DayFiles>,
}

impl Files {
    fn extract(dir: &Path) -> anyhow::Result<Self> {
        let mut days = BTreeMap::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?.path();
            if !entry.is_dir() {
                continue;
            }
            if let Ok(day_files) = DayFiles::extract(&entry) {
                let day = entry
                    .components()
                    .last()
                    .expect("there should be at least one component");
                let day = u32::from_str_radix(day.as_os_str().to_string_lossy().as_ref(), 10)?;
                days.insert(day, day_files);
            }
        }
        Ok(Files { days })
    }
}
fn run_part(
    part_letter: &str,
    part: &PartFiles,
    solve: impl Fn(&str) -> anyhow::Result<String>,
) -> anyhow::Result<()> {
    println!("Part {}", part_letter);
    if let Some(input) = &part.input {
        let input = fs::read(input)?;
        match solve(&String::from_utf8_lossy(&input)) {
            Ok(x) => println!("  \x1b[1;37m{}\x1b[0m", x),
            Err(e) => println!("  \x1b[0;31m{}\x1b[0m", e),
        }
    }
    println!("Tests");
    for test_file in &part.tests {
        let input = fs::read(&test_file.input)?;
        let output = fs::read(&test_file.output)?;
        let input = String::from_utf8_lossy(&input);
        let output = String::from_utf8_lossy(&output).trim().to_string();
        let pass = match solve(&input) {
            Ok(x) if x == output => Ok(()),
            Ok(x) => Err(anyhow!("{} != {}", x, output)),
            Err(e) => Err(e),
        };
        print!("  {:0>3} ", test_file.number);
        match pass {
            Ok(_) => println!("\x1b[0;32m☑\x1b[0m"),
            Err(e) => {
                println!("\x1b[0;31m☒\x1b[0m");
                println!("    \x1b[0;31m{}\x1b[0m", e)
            }
        }
    }
    Ok(())
}

fn handle_day(day: u32, day_files: &DayFiles) -> anyhow::Result<()> {
    if let Some(solution) = advent2024::Solution::at(day) {
        println!("Day {:0>2}", day);
        run_part("A", &day_files.part_a, |x| solution.part1(x))?;
        if let Some(part) = day_files.part_b.as_ref() {
            run_part("B", part, |x| solution.part2(x))?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse(&std::env::args().collect::<Vec<_>>())?;
    let files = Files::extract(&args.path)?;
    match args.which_days {
        WhichDays::All => {
            let mut i = 0;
            for (day, day_files) in files.days {
                if i > 0 {
                    println!("");
                }
                handle_day(day, &day_files)?;
                i += 1;
            }
        }
        WhichDays::Just(day) => {
            let day_files = files
                .days
                .get(&day)
                .ok_or(anyhow!("missing files for day {:0>2}", day))?;
            handle_day(day, &day_files)?;
        }
    }
    Ok(())
}
