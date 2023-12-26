use crate::vec3::*;
use anyhow::*;
use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Space {
    grid: Vec<[[Option<usize>; 32]; 32]>,
}

impl Space {
    pub fn resting_positions(bricks: &mut [Brick]) -> Result<Self> {
        bricks.sort_by_key(|brick| brick.start.z);
        let mut grid: Vec<[[Option<usize>; 32]; 32]> = vec![];

        for (i, brick) in bricks.iter_mut().enumerate() {
            let resting_z: usize = (0..=brick.start.z)
                .rev()
                .find_or_last(|&z| {
                    let resting = grid
                        .get(z as usize)
                        .is_some_and(|plane| brick.shadow().any(|(x, y)| plane[x][y].is_some()));
                    resting
                })
                .ok_or_else(|| anyhow!("brick {} has no resting position", i))?
                .try_into()?;

            let resting_z = resting_z + 1;
            let ceiling = resting_z + brick.size.z as usize;

            brick.start.z = resting_z as isize;

            while grid.len() <= ceiling {
                grid.push([[None; 32]; 32]);
            }
            for z in resting_z..=ceiling {
                for (x, y) in brick.shadow() {
                    grid[z][x][y] = Some(i as usize);
                }
            }
        }

        Ok(Self { grid })
    }

    pub fn supporting(&self, brick_id: usize, bricks: &[Brick]) -> HashSet<usize> {
        let mut supported_by_brick = HashSet::new();
        let brick = bricks[brick_id];
        let Self { grid } = self;
        let floor_above_brick = brick.start.z + brick.size.z + 1;

        if floor_above_brick >= grid.len() as isize {
            return supported_by_brick;
        }

        for (x, y) in brick.shadow() {
            if let Some(supporting_brick_id) = grid[floor_above_brick as usize][x][y] {
                supported_by_brick.insert(supporting_brick_id);
            }
        }

        let exclusively_supported_by_brick = supported_by_brick
            .iter()
            .copied()
            .filter(|&brick_id_below| {
                let supporting_brick = bricks[brick_id_below];
                let exclusive = supporting_brick.shadow().all(|(x, y)| {
                    let pos = grid[floor_above_brick as usize - 1][x][y];
                    pos.is_none() || pos == Some(brick_id)
                });

                exclusive
            })
            .collect();

        exclusively_supported_by_brick
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Brick {
    pub start: V3,
    pub size: V3,
}

impl Brick {
    pub fn shadow(&self) -> impl Iterator<Item = (usize, usize)> + Clone + '_ {
        let Self { start, size } = self;
        let (x, y) = (start.x..=start.x + size.x, start.y..=start.y + size.y);

        x.cartesian_product(y)
            .map(|(x, y)| (x as usize, y as usize))
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1,2,3~4,5,6
        let err = || anyhow!("invalid brick: {}", s);
        let (from, to) = s.split_once('~').ok_or_else(err)?;

        let start = from.parse()?;
        let to: V3 = to.parse()?;

        let size = to - start;

        Ok(Self { start, size })
    }
}
