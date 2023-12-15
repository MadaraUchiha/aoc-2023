use anyhow::*;
use aoc_2023::*;

struct Day;

mod lens_box;
use itertools::Itertools;
use lens_box::*;

impl BasicSolution for Day {
    type Parsed = Vec<&'static str>;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 1320;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 145;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.iter().map(|s| hash(s) as usize).sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let mut lens_box = LensBox::new();
        let instructions: Vec<_> = input
            .iter()
            .map(|s| Instruction::try_from(*s))
            .try_collect()?;

        lens_box.run_instructions(&instructions);

        Ok(lens_box.focus_power())
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
        Ok(data.split(",").collect())
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
