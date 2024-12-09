#[derive(Debug)]
enum Instr {
    Mul(i64, i64),
    Do,
    Dont,
}
#[derive(Debug)]
struct Data {
    instrs: Vec<Instr>,
}

peg::parser! {
    grammar data_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("failed to parse number")) }

        rule mul() -> Instr
            = "mul(" a:number() "," b:number() ")" { Instr::Mul(a, b) }

        rule do() -> Instr
            = "do()" { Instr::Do }

        rule dont() -> Instr
            = "don't()" { Instr::Dont }

        rule instr() -> Instr
            = mul()
            / do()
            / dont()

        rule instr_or_skip() -> Option<Instr>
            = i:instr() { Some(i) }
            / !instr() [_] { None }

        pub rule root() -> Data
            = instrs:(instr_or_skip())* { Data { instrs: instrs.into_iter().filter_map(|x| x).collect() }}
    }
}

impl Data {
    fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(data_parser::root(input)?)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Solution;

impl super::RawSolution for Solution {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let data = Data::parse(input)?;
        let mut ans: i64 = 0;
        for i in data.instrs {
            if let Instr::Mul(a, b) = i {
                ans += a * b;
            }
        }
        Ok(ans.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let data = Data::parse(input)?;
        let mut on = true;
        let mut ans: i64 = 0;
        for i in data.instrs {
            match i {
                Instr::Do => on = true,
                Instr::Dont => on = false,
                Instr::Mul(a, b) if on => ans += a * b,
                _ => {}
            }
        }
        Ok(ans.to_string())
    }
}
