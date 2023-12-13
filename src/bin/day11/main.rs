use anyhow::*;
use aoc_2023::*;

struct Day;

mod universe;
use itertools::Itertools;
use universe::*;

impl BasicSolution for Day {
    type Parsed = Universe;
    type Answer = u64;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 374;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 82000210;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| input.distance(a, b, 2))
            .sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| input.distance(a, b, 1_000_000))
            .sum())
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.parse()
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
