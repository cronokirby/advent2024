use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Data {
    numbers: Vec<(i64, i64)>,
}

peg::parser! {
    grammar data_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("failed to parse number")) }

        rule pair() -> (i64, i64)
            = a:number() (" ")+ b:number() { (a, b) }

        pub rule root() -> Data
            = numbers:(pair() ** "\n") "\n"* { Data { numbers } }
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
        let len = data.numbers.len();
        let mut left = Vec::with_capacity(len);
        let mut right = Vec::with_capacity(len);
        for (l, r) in data.numbers {
            left.push(l);
            right.push(r);
        }
        left.sort_unstable();
        right.sort_unstable();

        let answer: i64 = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(x, y)| (y - x).abs())
            .sum();

        Ok(answer.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let data = Data::parse(input)?;
        let mut left = Vec::with_capacity(data.numbers.len());
        let mut right = BTreeMap::new();
        for (l, r) in data.numbers {
            left.push(l);
            *right.entry(r).or_insert_with(|| 0) += 1;
        }
        let answer: i64 = left
            .into_iter()
            .map(|x| x * right.get(&x).copied().unwrap_or(0))
            .sum();

        Ok(answer.to_string())
    }
}
