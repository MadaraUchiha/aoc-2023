use anyhow::*;
use aoc_2023::*;

struct Day;

impl BasicSolution for Day {
    type Parsed = &'static str;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        todo!()
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        todo!()
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        todo!()
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
