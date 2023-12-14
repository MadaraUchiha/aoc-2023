use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};

pub const NORTH: (isize, isize) = (0, -1);
pub const SOUTH: (isize, isize) = (0, 1);
pub const EAST: (isize, isize) = (1, 0);
pub const WEST: (isize, isize) = (-1, 0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RockKind {
    Round,
    Square,
}
use RockKind::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    height: usize,
    width: usize,
    pub rocks: HashMap<(usize, usize), RockKind>,
}

impl Platform {
    pub fn calculate_load(&self) -> usize {
        let round_rocks = self.rocks.iter().filter(|(_, kind)| **kind == Round);
        let loads = round_rocks.map(|((_, y), _)| self.height - y);

        loads.sum()
    }
    pub fn fully_rotate_tilt(&self) -> Self {
        let mut past_results = HashMap::new();
        let mut past_results_index = Vec::new();
        let mut current = self.clone();
        let mut i = 0;
        let (start, end) = loop {
            if let Some((start, _)) = past_results.get(&current) {
                break (*start, i);
            }

            let next = current.rotate_tilt();
            past_results.insert(current.clone(), (i, next.clone()));
            past_results_index.push(current);
            current = next;
            i += 1;
        };

        let loop_length = end - start;
        let offset = 1_000_000_000 - start;
        let index = offset % loop_length + start;

        past_results_index[index].clone()
    }
    /// Returns a new platform after a single full rotation cycle
    pub fn rotate_tilt(&self) -> Self {
        self.fully_tilt(NORTH)
            .fully_tilt(WEST)
            .fully_tilt(SOUTH)
            .fully_tilt(EAST)
    }
    pub fn fully_tilt(&self, delta: (isize, isize)) -> Self {
        let mut current = self.clone();
        let mut result = current.tilt(delta);
        while result != current {
            current = result;
            result = current.tilt(delta);
        }
        result
    }
    fn tilt(&self, (dx, dy): (isize, isize)) -> Self {
        let mut new_rocks = HashMap::new();

        for (&(x, y), kind) in &self.rocks {
            if *kind == Square {
                new_rocks.insert((x, y), kind.clone());
                continue;
            }
            let new_y = y.saturating_add_signed(dy).clamp(0, self.height - 1);
            let new_x = x.saturating_add_signed(dx).clamp(0, self.width - 1);
            if let None = self.rocks.get(&(new_x, new_y)) {
                new_rocks.insert((new_x, new_y), kind.clone());
                continue;
            }
            new_rocks.insert((x, y), kind.clone());
        }

        Self {
            height: self.height,
            width: self.width,
            rocks: new_rocks,
        }
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();

        let enumerated_lines = lines.enumerate();
        let rocks = enumerated_lines
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let kind = match c {
                        'O' => Some(Round),
                        '#' => Some(Square),
                        _ => None,
                    };

                    kind.map(|kind| ((x, y), kind))
                })
            })
            .collect();

        Ok(Self {
            height,
            rocks,
            width,
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let kind = self.rocks.get(&(x, y));
                let c = match kind {
                    Some(Round) => 'O',
                    Some(Square) => '#',
                    None => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.height.hash(state);
        self.width.hash(state);
        self.rocks.iter().for_each(|(coord, kind)| {
            coord.hash(state);
            kind.hash(state);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_hash_works() {
        let mut platform1 = Platform {
            height: 1,
            width: 1,
            rocks: HashMap::new(),
        };

        let mut platform2 = Platform {
            height: 1,
            width: 1,
            rocks: HashMap::new(),
        };

        platform1.rocks.insert((0, 0), Round);
        platform1.rocks.insert((1, 0), Square);

        platform2.rocks.insert((0, 0), Round);
        platform2.rocks.insert((1, 0), Square);

        let mut map = HashMap::new();

        map.insert(platform1, 1);
        assert_eq!(map.get(&platform2), Some(&1));
    }
}
