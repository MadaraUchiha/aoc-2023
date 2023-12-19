use anyhow::*;
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashSet},
    ops::Sub,
    str::FromStr,
};

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
    pub fn find_best_lava_path<const MIN_STEPS: usize, const MAX_STEPS: usize>(
        &self,
    ) -> Option<usize> {
        let start = Vector2D(0, 0);
        let end = Vector2D(
            self.cost_map.len() as isize - 1,
            self.cost_map[0].len() as isize - 1,
        );
        let mut frontier = BinaryHeap::new();
        let mut visited = HashSet::new();

        frontier.push(LavaFlowStepCost(
            0,
            LavaFlowStep {
                point: start,
                direction: Vector2D(0, 0),
                in_a_row: 0,
            },
        ));

        while let Some(LavaFlowStepCost(step_cost, step)) = frontier.pop() {
            let LavaFlowStep {
                point, in_a_row, ..
            } = step;

            if !visited.insert(step) {
                continue;
            }

            if in_a_row >= MAX_STEPS {
                // This step is invalid, it goes over the maximum
                continue;
            }

            if point == end && in_a_row >= MIN_STEPS {
                // We reached the end, and we made enough steps to reach it
                return Some(step_cost);
            }

            for next in self.adjacent(&point) {
                let next_cost = match self.get(&next) {
                    Some(cost) => cost,
                    None => continue, // out of bounds
                };

                let next_direction = next - point;
                let must_cotinue_straight = in_a_row + 1 < MIN_STEPS;
                let going_straight =
                    next_direction == step.direction || step.direction == Vector2D(0, 0);
                let going_reverse = next_direction == step.direction.reverse();

                if must_cotinue_straight && !going_straight {
                    continue; // don't consider steps other than going straight
                }

                if going_reverse {
                    continue; // don't go back
                }

                let next_step = LavaFlowStep {
                    point: next,
                    direction: next_direction,
                    in_a_row: if going_straight { in_a_row + 1 } else { 0 },
                };
                frontier.push(LavaFlowStepCost(step_cost + next_cost, next_step));
            }
        }

        None
    }

    pub fn get(&self, point: &Vector2D) -> Option<usize> {
        let (x, y) = point.get()?;

        self.cost_map.get(y)?.get(x).map(|&c| c.into())
    }

    pub fn adjacent(&self, point: &Vector2D) -> Vec<Vector2D> {
        vec![
            Vector2D(point.0 + 1, point.1),
            Vector2D(point.0, point.1 + 1),
            Vector2D(point.0 - 1, point.1),
            Vector2D(point.0, point.1 - 1),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2D(pub isize, pub isize);

impl Vector2D {
    pub fn get(&self) -> Option<(usize, usize)> {
        if self.0.is_negative() || self.1.is_negative() {
            return None;
        }
        Some((self.0 as usize, self.1 as usize))
    }

    pub fn reverse(&self) -> Vector2D {
        Vector2D(-self.0, -self.1)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LavaFlowStep {
    point: Vector2D,
    direction: Vector2D,
    in_a_row: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LavaFlowStepCost(usize, LavaFlowStep);

impl PartialOrd for LavaFlowStepCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|o| o.reverse())
    }
}

impl Ord for LavaFlowStepCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}
