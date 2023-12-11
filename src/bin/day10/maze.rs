use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn check(&self, dir: &Direction) -> Option<Self> {
        let &Point { x, y } = self;
        let (new_x, new_y) = match dir {
            Direction::North => (x, y.saturating_sub(1)),
            Direction::East => (x.saturating_add(1), y),
            Direction::South => (x, y.saturating_add(1)),
            Direction::West => (x.saturating_sub(1), y),
        };
        if new_x == x && new_y == y {
            return None;
        }
        Some(Point { x: new_x, y: new_y })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map(HashMap<Point, Pipe>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maze {
    pub map: Map,
    pub height: u8,
    pub width: u8,
    pub start_position: Point,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_position = None;
        let mut map = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Point {
                    x: x as u8,
                    y: y as u8,
                };
                if c == 'S' {
                    start_position = Some(pos.clone());
                    continue;
                }
                let pipe = Pipe::try_from(c)?;
                map.insert(pos, pipe);
            }
        }

        let map = Map(map);

        let start_position = start_position
            .ok_or_else(|| anyhow!("No starting position found"))?
            .clone();

        let height = map.0.keys().map(|p| p.y).max().unwrap() + 1;
        let width = map.0.keys().map(|p| p.x).max().unwrap() + 1;

        Ok(Maze {
            map,
            width,
            height,
            start_position,
        })
    }
}

impl Maze {
    pub fn get(&self, point: &Point) -> Option<&Pipe> {
        self.map.0.get(point)
    }

    pub fn find_start_type(&mut self) -> Option<Pipe> {
        let around = self.map.around(&self.start_position);

        let connecting = around.into_iter().filter_map(|(dir, pipe)| {
            let connects_to = pipe.connects_to()?;
            if connects_to.contains(&dir.op()) {
                Some(dir)
            } else {
                None
            }
        });

        let pipe = Pipe::from_directions(&connecting.collect_vec());

        self.map
            .0
            .insert(self.start_position.clone(), pipe.clone()?);

        pipe
    }

    pub fn traverse_loop(&self) -> Option<Vec<Point>> {
        let mut current_position = self.start_position;
        let mut current_direction = self.get(&current_position)?.connects_to()?.get(0)?.clone();
        let mut path: Vec<Point> = Vec::new();

        loop {
            path.push(current_position.clone());

            let next_position = current_position.check(&current_direction)?;
            let next_pipe = self.get(&next_position)?;
            current_position = next_position;
            current_direction = next_pipe
                .connects_to()?
                .iter()
                .find(|dir| dir.op() != current_direction)?
                .clone();

            if current_position == self.start_position {
                return Some(path);
            }
        }
    }

    pub fn count_tiles_in_loop(&self) -> Option<Vec<Point>> {
        let polygon: HashSet<_> = HashSet::from_iter(self.traverse_loop()?);
        // let mut count = 0;
        let mut points = Vec::new();
        for y in 0..self.height {
            let mut state = TraversalState::OutsideLoop;
            for x in 0..self.width {
                let point = Point { x, y };
                let pipe = self.get(&point)?;
                if polygon.contains(&point) {
                    state = state.next(pipe.clone());
                } else if state == InsideLoop {
                    points.push(point);
                }
            }
        }

        Some(points)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn op(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pipe {
    NorthSouth,
    EastWest,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
    Ground,
}

impl Pipe {
    fn connects_to(&self) -> Option<[Direction; 2]> {
        match self {
            Self::NorthSouth => Some([Direction::North, Direction::South]),
            Self::EastWest => Some([Direction::East, Direction::West]),
            Self::SouthEast => Some([Direction::East, Direction::South]),
            Self::SouthWest => Some([Direction::South, Direction::West]),
            Self::NorthEast => Some([Direction::North, Direction::East]),
            Self::NorthWest => Some([Direction::North, Direction::West]),
            Self::Ground => None,
        }
    }

    fn from_direction(dir: &Direction) -> Vec<Self> {
        static PIPES: [Pipe; 6] = [
            Pipe::NorthSouth,
            Pipe::EastWest,
            Pipe::SouthEast,
            Pipe::SouthWest,
            Pipe::NorthEast,
            Pipe::NorthWest,
        ];
        PIPES
            .into_iter()
            .filter(|pipe| pipe.connects_to().unwrap().contains(dir))
            .collect_vec()
    }

    fn from_directions(dirs: &[Direction]) -> Option<Self> {
        dirs.iter()
            .map(Self::from_direction)
            .reduce(|a, b| a.into_iter().filter(|pipe| b.contains(pipe)).collect_vec())
            .map(|vec| vec[0])
            .map(|pipe| pipe.clone())
    }
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '.' => Self::Ground,
            _ => Err(anyhow!("Invalid pipe type: {}", value))?,
        })
    }
}

impl Map {
    fn around(&self, point: &Point) -> Vec<(Direction, Pipe)> {
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        directions
            .into_iter()
            .filter_map(|dir| Some((dir, point.check(&dir)?)))
            .filter_map(|(dir, pos)| Some((dir, self.0.get(&pos)?.clone())))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TraversalState {
    OutsideLoop,
    InsideLoop,
    OnTopBorder,
    OnBottomBorder,
}

use Pipe::*;
use TraversalState::*;

impl TraversalState {
    fn next(self, pipe: Pipe) -> Self {
        match (self, pipe) {
            (OutsideLoop, NorthSouth) => InsideLoop,
            (OutsideLoop, NorthEast) => OnBottomBorder,
            (OutsideLoop, SouthEast) => OnTopBorder,
            (InsideLoop, NorthSouth) => OutsideLoop,
            (InsideLoop, NorthEast) => OnTopBorder,
            (InsideLoop, SouthEast) => OnBottomBorder,
            (OnTopBorder, EastWest) => OnTopBorder,
            (OnTopBorder, NorthWest) => InsideLoop,
            (OnTopBorder, SouthWest) => OutsideLoop,
            (OnBottomBorder, EastWest) => OnBottomBorder,
            (OnBottomBorder, NorthWest) => OutsideLoop,
            (OnBottomBorder, SouthWest) => InsideLoop,
            _ => panic!("Invalid state transition: {:?} -> {:?}", self, pipe),
        }
    }
}
