use std::{collections::VecDeque, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Ground,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}
use Tile::*;

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "#" => Tile::Wall,
            "." => Tile::Ground,
            "<" => Tile::SlopeLeft,
            ">" => Tile::SlopeRight,
            "^" => Tile::SlopeUp,
            "v" => Tile::SlopeDown,
            _ => bail!("Invalid tile: {}", s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Garden {
    tiles: Vec<Vec<Tile>>,
}

impl Garden {
    pub fn measure_longest_path<const SLIPPER_SLOPES: bool>(&self) -> usize {
        let end = (self.tiles[0].len() - 2, self.tiles[0].len() - 1);
        let mut frontier = VecDeque::new();
        frontier.push_back(Step::new(1, 0));
        let mut longest = 0;

        while let Some(mut step) = frontier.pop_front() {
            if (step.x, step.y) == end {
                if step.visited.len() > longest {
                    longest = step.visited.len();
                }
                continue;
            }
            let candidates = self.valid_adjacent::<SLIPPER_SLOPES>(step.x, step.y);

            if candidates.len() == 2 {
                let candidate = candidates
                    .into_iter()
                    .find(|(x, y)| !step.visited.contains(&(*x, *y)));

                match candidate {
                    Some(candidate) => {
                        step.step(candidate);
                        frontier.push_back(step);
                    }
                    None => {
                        continue;
                    }
                }
            } else {
                for candidate in candidates {
                    if step.visited.contains(&candidate) {
                        continue;
                    }

                    let mut new_step = step.clone();
                    new_step.step(candidate);

                    frontier.push_back(new_step);
                }
            }
        }

        longest - 1 // not including start
    }

    fn valid_adjacent<const SLIPPERY_SLOPES: bool>(
        &self,
        x: usize,
        y: usize,
    ) -> Vec<(usize, usize)> {
        if SLIPPERY_SLOPES {
            match self.tiles[y][x] {
                SlopeLeft => return vec![(x - 1, y)],
                SlopeRight => return vec![(x + 1, y)],
                SlopeUp => return vec![(x, y - 1)],
                SlopeDown => return vec![(x, y + 1)],
                _ => {}
            }
        }
        let candidates = match self.tiles[y][x] {
            Wall => vec![],
            _ => {
                let mut directions = vec![];

                if x > 0 {
                    directions.push((x - 1, y));
                }
                if x < self.tiles.len() - 1 {
                    directions.push((x + 1, y));
                }
                if y > 0 {
                    directions.push((x, y - 1));
                }
                if y < self.tiles[0].len() - 1 {
                    directions.push((x, y + 1));
                }

                directions
            }
        };

        candidates
            .into_iter()
            .filter(|(x, y)| self.tiles[*y][*x] != Wall)
            .collect()
    }
}

impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_string().parse()).try_collect())
            .try_collect()?;

        Ok(Self { tiles })
    }
}

#[derive(Debug, Clone)]
struct Step {
    x: usize,
    y: usize,
    visited: Vec<(usize, usize)>,
}
impl Step {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            visited: vec![(x, y)],
        }
    }
    fn step(&mut self, to: (usize, usize)) {
        self.x = to.0;
        self.y = to.1;
        self.visited.push(to);
    }
}
