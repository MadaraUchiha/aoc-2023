use anyhow::*;
use aoc_2023::*;
use crucible::{Field, Vector2D};

mod crucible;

struct Day;

impl BasicSolution for Day {
    type Parsed = Field;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 102;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let start = Vector2D(0, 0);
        let end = Vector2D(
            input.cost_map.len() as isize - 1,
            input.cost_map[0].len() as isize - 1,
        );
        input
            .find_best_lava_path(start, end)
            .ok_or_else(|| anyhow!("No path found"))
    }

    fn part2(_input: Self::Parsed) -> Result<Self::Answer> {
        todo!()
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
