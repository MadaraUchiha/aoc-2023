use anyhow::*;
use aoc_2023::*;
use rocks::{Platform, NORTH};

mod rocks;

struct Day;

impl BasicSolution for Day {
    type Parsed = Platform;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 136;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 64;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let result = input.fully_tilt(NORTH);
        Ok(result.calculate_load())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let result = input.fully_rotate_tilt();
        Ok(result.calculate_load())
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
