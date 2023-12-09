use std::str::FromStr;

use anyhow::*;
use aoc_2023::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Sequence(Vec<i32>);

struct Day;

impl BasicSolution for Day {
    type Parsed = Vec<Sequence>;
    type Answer = i32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 114;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.into_iter().map(|s| s.next_in_sequence()).sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .into_iter()
            .map(|s| s.reverse())
            .map(|s| s.next_in_sequence())
            .sum())
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.lines().map(|s| s.parse()).collect()
    }
}

impl FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            line.split_ascii_whitespace()
                .map(|s| s.parse())
                .try_collect()?,
        ))
    }
}

impl Sequence {
    fn differences(&self) -> Self {
        Self(self.0.iter().tuple_windows().map(|(a, b)| b - a).collect())
    }

    fn reverse(self) -> Self {
        Self(self.0.into_iter().rev().collect())
    }

    fn next_in_sequence(self) -> i32 {
        let mut result = 0;
        let mut current_sequence = self;
        loop {
            result += current_sequence.0.iter().last().unwrap();
            if current_sequence.0.iter().all(|&n| n == 0) {
                return result;
            }
            current_sequence = current_sequence.differences();
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
