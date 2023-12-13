use anyhow::*;
use aoc_2023::*;
use itertools::Itertools;
use springs::SpringFormation;

mod springs;

struct Day;

impl BasicSolution for Day {
    type Parsed = Vec<SpringFormation>;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 21;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 525152;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .into_iter()
            .map(|formation| formation.count_arrangements())
            .sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .into_iter()
            .map(|formation| formation.unfold())
            .map(|formation| formation.count_arrangements())
            .sum())
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.lines().map(str::parse).try_collect()
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
