use std::collections::HashSet;

use anyhow::*;
use aoc_2023::*;
use energizer::{Energizer, Ray};
use rayon::prelude::*;
mod energizer;

struct Day;

impl BasicSolution for Day {
    type Parsed = Energizer;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 46;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 51;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(count_visited_tiles(&input, Ray::new(0, 0, 1, 0)))
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("No solution found");

        let southward_rays = input.grid[0]
            .iter()
            .enumerate()
            .map(|(x, _)| Ray::new(x, 0, 0, 1));
        let northward_rays = input.grid[input.grid.len() - 1]
            .iter()
            .enumerate()
            .map(|(x, _)| Ray::new(x, input.grid.len() - 1, 0, -1));
        let eastward_rays = input
            .grid
            .iter()
            .enumerate()
            .map(|(y, _)| Ray::new(0, y, 1, 0));
        let westward_rays = input
            .grid
            .iter()
            .enumerate()
            .map(|(y, _)| Ray::new(input.grid[0].len() - 1, y, -1, 0));

        let all_rays = southward_rays
            .chain(northward_rays)
            .chain(eastward_rays)
            .chain(westward_rays);

        all_rays
            // Look ma, parallelism!
            .par_bridge()
            .map(|ray| count_visited_tiles(&input, ray))
            .max()
            .ok_or_else(err)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.parse()
    }
}

pub fn main() -> anyhow::Result<()> {
    Day::main()
}

fn count_visited_tiles(energizer: &Energizer, first_ray: Ray) -> usize {
    energizer.iter(first_ray).collect::<HashSet<_>>().len()
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
