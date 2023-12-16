use anyhow::*;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    EmptySpace,
    LeftRightSplitter,
    UpDownSplitter,
    LeftRightMirror,
    RightLeftMirror,
}
use Tile::*;

#[derive(Debug, Clone)]
pub struct Energizer {
    pub grid: Vec<Vec<Tile>>,
}

impl FromStr for Energizer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(EmptySpace),
                        '-' => Ok(LeftRightSplitter),
                        '|' => Ok(UpDownSplitter),
                        '/' => Ok(RightLeftMirror),
                        '\\' => Ok(LeftRightMirror),
                        _ => Err(anyhow!("Invalid character: {}", c)),
                    })
                    .try_collect()
            })
            .try_collect()?;

        Ok(Self { grid })
    }
}

impl Energizer {
    pub fn iter(&self, first_ray: Ray) -> EnergizerIter {
        EnergizerIter::new(self, first_ray)
    }
}

pub struct EnergizerIter<'a> {
    energizer: &'a Energizer,
    rays: VecDeque<Ray>,
    visited: HashSet<Ray>,
}

impl<'a> EnergizerIter<'a> {
    pub fn new(energizer: &'a Energizer, first_ray: Ray) -> Self {
        let rays = VecDeque::from(vec![first_ray]);
        let visited = HashSet::new();

        Self {
            energizer,
            rays,
            visited,
        }
    }
}

impl Iterator for EnergizerIter<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let ray = self.rays.pop_front()?;
        let tile = self
            .energizer
            .grid
            .get(ray.y)
            .and_then(|row| row.get(ray.x));

        if let Some(_) = self.visited.get(&ray) {
            return self.next();
        }

        match tile {
            None => {
                // We've popped the ray off the queue, so now it's gone
                // Continue with the next ray
                return self.next();
            }
            Some(EmptySpace) => {
                ray.advance().map(|ray| self.rays.push_back(ray));
            }
            Some(splitter @ (LeftRightSplitter | UpDownSplitter)) => {
                let rays = ray.split(splitter);

                for ray in rays {
                    ray.advance().map(|ray| self.rays.push_back(ray));
                }
            }
            Some(mirror @ (LeftRightMirror | RightLeftMirror)) => {
                ray.mirror(mirror)
                    .advance()
                    .map(|ray| self.rays.push_back(ray));
            }
        }

        self.visited.insert(ray);
        Some((ray.x, ray.y))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ray {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

impl Ray {
    pub fn new(x: usize, y: usize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }
    fn advance(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add_signed(self.dx)?,
            y: self.y.checked_add_signed(self.dy)?,
            dx: self.dx,
            dy: self.dy,
        })
    }

    fn split(&self, splitter: &Tile) -> Vec<Ray> {
        let horizontal = self.dx != 0;
        match (splitter, horizontal) {
            (LeftRightSplitter, true) | (UpDownSplitter, false) => {
                vec![*self]
            }
            (LeftRightSplitter, false) => {
                let &Ray { x, y, .. } = self;
                let next1 = Ray { x, y, dx: 1, dy: 0 };
                let next2 = Ray {
                    x,
                    y,
                    dx: -1,
                    dy: 0,
                };
                vec![next1, next2]
            }
            (UpDownSplitter, true) => {
                let &Ray { x, y, .. } = self;
                let next1 = Ray { x, y, dx: 0, dy: 1 };
                let next2 = Ray {
                    x,
                    y,
                    dx: 0,
                    dy: -1,
                };
                vec![next1, next2]
            }

            _ => unreachable!(),
        }
    }

    fn mirror(&self, mirror: &Tile) -> Self {
        let horizontal = self.dx != 0;
        match (mirror, horizontal) {
            (LeftRightMirror, true) | (RightLeftMirror, false) => self.rotate_clockwise(),
            (LeftRightMirror, false) | (RightLeftMirror, true) => self.rotate_counterclockwise(),
            _ => unreachable!(),
        }
    }

    fn rotate_clockwise(&self) -> Self {
        let (dx, dy) = match (self.dx, self.dy) {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => unreachable!(),
        };

        Self {
            x: self.x,
            y: self.y,
            dx,
            dy,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        let (dx, dy) = match (self.dx, self.dy) {
            (1, 0) => (0, -1),
            (0, 1) => (1, 0),
            (-1, 0) => (0, 1),
            (0, -1) => (-1, 0),
            _ => unreachable!(),
        };

        Self {
            x: self.x,
            y: self.y,
            dx,
            dy,
        }
    }
}
