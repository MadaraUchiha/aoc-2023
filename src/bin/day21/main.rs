use std::{
    collections::HashSet,
    str::{from_utf8, FromStr},
};

use anyhow::*;
use aoc_2023::{
    util::{grid::*, point::*},
    *,
};

const GROUND: u8 = b'.';
const PART_2_STEPS: usize = 26501365;

struct Day;

impl BasicSolution for Day {
    type Parsed = Garden;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 16;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 167004;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.count_default())
    }

    fn part2(mut garden: Self::Parsed) -> Result<Self::Answer> {
        garden.exapnd();
        let b0 = garden.count_reachable(garden.start, 65) as isize;
        let b1 = garden.count_reachable(garden.start, 65 + 131) as isize;
        let b2 = garden.count_reachable(garden.start, 65 + 131 + 131) as isize;

        let n = PART_2_STEPS / 131;

        let det_a: f64 = -2.0;
        let det_a0: f64 = -b0 as f64 + 2.0 * b1 as f64 - b2 as f64;
        let det_a1: f64 = 3.0 * b0 as f64 - 4.0 * b1 as f64 + b2 as f64;
        let det_a2: f64 = -2.0 * b0 as f64;
        let x0: usize = (det_a0 / det_a) as usize;
        let x1: usize = (det_a1 / det_a) as usize;
        let x2: usize = (det_a2 / det_a) as usize;
        Ok(x0 * n * n + x1 * n + x2)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.parse()
    }
}

pub fn main() -> anyhow::Result<()> {
    Day::main()
}

#[derive(Debug, Clone)]
struct Garden {
    steps: usize,
    map: Grid<u8>,
    start: Point,
}

impl Garden {
    fn count_default(&self) -> usize {
        self.count_reachable(self.start, self.steps)
    }
    fn count_reachable(&self, start: Point, steps: usize) -> usize {
        self.find_reachable(start, steps).len()
    }

    fn find_reachable(&self, start: Point, steps: usize) -> HashSet<Point> {
        let mut positions: HashSet<Point> = HashSet::new();
        positions.insert(start);

        for _ in 0..steps {
            let mut new_positions: HashSet<Point> = HashSet::new();
            for position in positions {
                for direction in ADJACENT {
                    let new_position = position + direction;
                    if self.map.in_bounds(new_position) && self.map[new_position] == GROUND {
                        new_positions.insert(new_position);
                    }
                }
            }
            positions = new_positions;
        }
        positions
    }

    fn exapnd(&mut self) {
        let mut new_map = String::new();
        for _ in 0..5 {
            let rows = self.map.data.chunks(self.map.width as usize);
            for row in rows {
                let row = from_utf8(row).unwrap();
                for _ in 0..5 {
                    new_map.push_str(row);
                }
                new_map.push('\n');
            }
        }
        self.start = Point::new(self.map.width * 5 / 2, self.map.height * 5 / 2);
        self.map = Grid::parse(&new_map);
    }
}

impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (steps, map) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("No steps found"))?;
        let steps = steps.parse()?;

        let mut map = Grid::parse(map);
        let start = map.find(b'S').ok_or_else(|| anyhow!("No start found"))?;
        map[start] = b'.';

        Ok(Self { steps, map, start })
    }
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
