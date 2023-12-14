use anyhow::*;
use aoc_2023::*;
use rocks::Grid;

mod rocks;

struct Day;

impl BasicSolution for Day {
    type Parsed = Grid;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 136;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 64;

    fn part1(mut input: Self::Parsed) -> Result<Self::Answer> {
        input.tilt_north();
        Ok(input.score())
    }

    fn part2(mut input: Self::Parsed) -> Result<Self::Answer> {
        let final_grid = input.tilt_cycle::<1_000_000_000>();
        Ok(final_grid.score())
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
