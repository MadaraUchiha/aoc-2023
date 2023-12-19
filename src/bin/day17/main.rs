use anyhow::*;
use aoc_2023::*;
use crucible::*;

mod crucible;

struct Day;

impl BasicSolution for Day {
    type Parsed = Field;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 102;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 94;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        input
            .find_best_lava_path::<0, 3>()
            .ok_or_else(|| anyhow!("No path found"))
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        input
            .find_best_lava_path::<4, 10>()
            .ok_or_else(|| anyhow!("No path found"))
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
