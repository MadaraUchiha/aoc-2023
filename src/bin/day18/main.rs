use anyhow::*;
use aoc_2023::*;
use itertools::Itertools;
use lagoon::*;

mod lagoon;

struct Day;

impl BasicSolution for Day {
    type Parsed = Vec<Instruction>;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 62;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 952408144115;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let mut lagoon = Lagoon::new();

        lagoon.dig_trench(input).count()
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let mut lagoon = Lagoon::new();

        lagoon.dig_trench(
            input
                .into_iter()
                .map(|i| i.into_correct_instruction())
                .try_collect()?,
        );

        lagoon.count()
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
        data.lines().map(|line| line.try_into()).collect()
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
