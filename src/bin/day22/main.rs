use anyhow::*;
use aoc_2023::*;
use bricks::*;

mod bricks;
mod vec3;

struct Day;

impl BasicSolution for Day {
    type Parsed = (Space, Vec<Brick>);
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 5;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 7;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let (space, bricks) = input;
        let mut total = 0;

        for brick_id in 0..bricks.len() {
            let supporting = space.supporting(brick_id, &bricks);
            if supporting.len() == 0 {
                total += 1;
            }
        }

        Ok(total)
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let (_, bricks) = input;

        let mut total = 0;

        for brick_idx in 0..bricks.len() {
            // the most idiotic thing I can think of :D
            let mut new_bricks = bricks.clone();

            new_bricks.remove(brick_idx);
            new_bricks.sort_by_key(|brick| brick.start.z);

            let mut new_bricks_after = new_bricks.clone();

            Space::resting_positions(&mut new_bricks_after)?;

            total += new_bricks_after
                .into_iter()
                .zip(new_bricks)
                .filter(|(a, b)| a != b)
                .count();
        }

        Ok(total)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        let mut bricks = data
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;
        let space = Space::resting_positions(&mut bricks)?;
        Ok((space, bricks))
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
