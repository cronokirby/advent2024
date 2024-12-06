#[derive(Debug)]
struct Data {
    levels: Vec<Vec<i64>>,
}

peg::parser! {
    grammar data_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("failed to parse number")) }

        rule row() -> Vec<i64>
            = number() ** " "

        rule matrix() -> Vec<Vec<i64>>
            = row() ** "\n"

        pub rule root() -> Data
            = levels:matrix() { Data { levels } }
    }
}

impl Data {
    fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(data_parser::root(input)?)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Increasing {
    Yes,
    No,
    Dunno,
}

impl Increasing {
    fn compatible(self, b: Increasing) -> bool {
        use Increasing::*;
        match (self, b) {
            (Dunno, _) => true,
            (_, Dunno) => true,
            (Yes, Yes) => true,
            (No, No) => true,
            _ => false,
        }
    }
}

fn safe(skip: impl Fn(usize) -> usize, row: &[i64]) -> bool {
    let mut increasing = Increasing::Dunno;
    let mut then = row[skip(0)];
    for i in 1..row.len() {
        let i = skip(i);
        if i >= row.len() {
            break;
        }
        let now = row[i];
        let diff = (now - then).abs();
        if diff > 3 || diff < 1 {
            return false;
        }

        let increasing_now = if now > then {
            Increasing::Yes
        } else {
            Increasing::No
        };
        if !increasing_now.compatible(increasing) {
            return false;
        }
        increasing = increasing_now;

        then = now;
    }
    true
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Solution;

impl super::RawSolution for Solution {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let data = Data::parse(input)?;
        let answer: usize = data.levels.iter().filter(|x| safe(|i| i, x)).count();
        Ok(answer.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let data = Data::parse(input)?;
        let answer: usize = data
            .levels
            .iter()
            .filter(|x| (0..x.len()).any(|i| safe(|j| if j >= i { j + 1 } else { j }, x)))
            .count();
        Ok(answer.to_string())
    }
}
