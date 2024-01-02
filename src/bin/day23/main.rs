use anyhow::*;
use aoc_2023::*;
use garden::*;

mod garden;
struct Day;

impl BasicSolution for Day {
    type Parsed = Garden;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 94;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 154;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.longest_distance())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.longest_distance_undirected())
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
