use anyhow::*;
use itertools::Itertools;
use std::{collections::BinaryHeap, ops::Sub, str::FromStr};

#[derive(Debug, Clone)]
pub struct Field {
    pub cost_map: Vec<Vec<u8>>,
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Invalid field");

        let cost_map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|d| d as u8).ok_or_else(err))
                    .try_collect()
            })
            .try_collect()?;

        Ok(Self { cost_map })
    }
}

impl Field {
    pub fn find_best_lava_path(&self, start: Vector2D, end: Vector2D) -> Option<usize> {
        let path_3_consecutive =
            |next: &Vector2D, last: &Vector2D, came_from: &Vec<Vec<Option<Vector2D>>>| {
                let two_ago = came_from[last.1 as usize][last.0 as usize];
                let two_ago = match two_ago {
                    Some(last) => last,
                    None => return false,
                };
                let three_ago = came_from[two_ago.1 as usize][two_ago.0 as usize];
                let three_ago = match three_ago {
                    Some(before_last) => before_last,
                    None => return false,
                };

                let next_direction = next - &last;
                let last_direction = last - &two_ago;
                let before_last_direction = &two_ago - &three_ago;

                last_direction == before_last_direction && last_direction == next_direction
            };

        let mut frontier = BinaryHeap::new();
        let mut costs_so_far = vec![vec![None; self.cost_map.len()]; self.cost_map[0].len()];
        let mut came_from = vec![vec![None; self.cost_map.len()]; self.cost_map[0].len()];

        let (x, y) = start.get()?;
        let start_cost = self.cost_map[y][x] as usize;
        frontier.push(PointCost {
            point: start,
            cost: start_cost,
        });
        costs_so_far[y][x] = Some(0);

        while let Some(PointCost { point, .. }) = frontier.pop() {
            if point == end {
                break;
            }

            for next in self.adjacent(&point) {
                if path_3_consecutive(&next, &point, &came_from) {
                    continue;
                }

                let next_cost = match self.get(&next) {
                    Some(cost) => cost,
                    None => continue,
                };

                let last_cost = costs_so_far[point.1 as usize][point.0 as usize]?;
                let next_cost = last_cost + next_cost;
                let (x, y) = next.get()?;
                let current_cost = costs_so_far[y][x];
                match current_cost {
                    Some(current_cost) if current_cost <= next_cost => continue,
                    _ => {
                        let distance_to_end = (end - next).0.abs() + (end - next).1.abs();
                        came_from[y][x] = Some(point);
                        costs_so_far[y][x] = Some(next_cost);
                        frontier.push(PointCost {
                            point: next,
                            cost: next_cost + distance_to_end as usize,
                        });
                    }
                }
            }
        }

        let Vector2D(x, y) = end;
        let x = x as usize;
        let y = y as usize;

        // print the winning path
        let mut path = vec![end];
        let mut current = came_from[y][x];
        while let Some(point) = current {
            path.push(point);
            if point == start {
                break;
            }
            let Vector2D(x, y) = point;
            current = came_from[y as usize][x as usize];
        }
        path.reverse();
        println!("{:?}", path);

        costs_so_far[y][x].into()
    }

    pub fn get(&self, point: &Vector2D) -> Option<usize> {
        let (x, y) = point.get()?;

        self.cost_map.get(y)?.get(x).map(|&c| c.into())
    }

    pub fn adjacent(&self, point: &Vector2D) -> Vec<Vector2D> {
        let adjacent = vec![
            Vector2D(point.0 - 1, point.1),
            Vector2D(point.0 + 1, point.1),
            Vector2D(point.0, point.1 - 1),
            Vector2D(point.0, point.1 + 1),
        ];

        adjacent
            .into_iter()
            .filter(|point| self.is_valid(point))
            .collect()
    }

    pub fn is_valid(&self, point: &Vector2D) -> bool {
        match point.get() {
            None => false,
            Some((x, y)) => x < self.cost_map.len() && y < self.cost_map[x].len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2D(pub isize, pub isize);

impl Vector2D {
    pub fn get(&self) -> Option<(usize, usize)> {
        if self.0.is_negative() || self.1.is_negative() {
            return None;
        }
        Some((self.0 as usize, self.1 as usize))
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for &Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointCost {
    point: Vector2D,
    cost: usize,
}

impl PartialOrd for PointCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|o| o.reverse())
    }
}

impl Ord for PointCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
