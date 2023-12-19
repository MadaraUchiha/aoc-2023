use anyhow::*;
use aoc_2023::*;
use maze::Maze;

struct Day;

mod maze;

impl BasicSolution for Day {
    type Parsed = Maze;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample_b.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 10;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("Failed to loop");
        let path = input.traverse_loop().ok_or_else(err)?;

        Ok(path.len() / 2)
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("Failed to count ground");
        let points = input.count_tiles_in_loop().ok_or_else(err)?;

        Ok(points)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        let mut data: Self::Parsed = data.parse()?;
        data.find_start_type();
        Ok(data)
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
