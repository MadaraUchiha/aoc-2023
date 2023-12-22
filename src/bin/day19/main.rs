use anyhow::*;
use aoc_2023::*;
use xmas::*;

mod xmas;

struct Day;

impl BasicSolution for Day {
    type Parsed = XMAS;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 19114;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 167409079868000;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("Failed to find sum");
        input.run().ok_or_else(err)
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.count_accepted_ranges())
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
        data.try_into()
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
