use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::*;
use aoc_2023::*;

struct Day;

impl BasicSolution for Day {
    type Parsed = Garden;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.find_reachable())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        todo!()
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
    map: HashMap<(usize, usize), Tile>,
    start: (usize, usize),
}

impl Garden {
    fn find_reachable(&self) -> usize {
        let mut frontier = VecDeque::new();
        let mut minimal_steps = HashMap::new();
        frontier.push_back((self.start, 0));
        minimal_steps.insert(self.start, 0);
        while let Some((pos, steps)) = frontier.pop_front() {
            if steps > 64 {
                continue;
            }

            for adj in adjacent(pos) {
                let Some(tile) = self.map.get(&adj) else {
                    continue;
                };
                if tile == &Tile::Rock {
                    continue;
                }
                if let None = minimal_steps.get(&adj) {
                    minimal_steps.insert(adj, steps + 1);
                    frontier.push_back((adj, steps + 1));
                }
            }
        }

        minimal_steps
            .into_iter()
            .filter(|(_, steps)| steps % 2 == 0)
            .count()
    }
}

fn adjacent((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    diffs
        .into_iter()
        .filter_map(move |(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
}
impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Ground,
                    '#' => Tile::Rock,
                    'S' => {
                        start = Some((x, y));
                        Tile::Ground
                    }
                    _ => bail!("invalid char"),
                };
                map.insert((x, y), tile);
            }
        }
        Ok(Self {
            map,
            start: start.ok_or_else(|| anyhow!("no start"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Rock,
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
