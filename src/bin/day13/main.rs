use anyhow::*;
use aoc_2023::*;
use field::Field;

mod field;

struct Day;

impl BasicSolution for Day {
    type Parsed = Vec<Field>;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 405;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 400;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .into_iter()
            .map(|field| field.find_all_reflections::<0>().unwrap())
            .sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .into_iter()
            .map(|field| field.find_all_reflections::<1>().unwrap())
            .sum())
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
        data.split("\n\n").map(|s| s.parse()).collect()
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
