use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rock {
    Round,
    Square,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    rocks: Vec<Vec<Option<Rock>>>,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.rocks[0].len()
    }

    pub fn height(&self) -> usize {
        self.rocks.len()
    }

    pub fn score(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, rock)| **rock == Some(Rock::Round))
                    .map(|_| self.height() - y)
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn tilt_cycle<const CYCLES: usize>(&mut self) -> Self {
        let mut cache: Vec<Grid> = Vec::new();
        let (start, end) = loop {
            if let Some((index, _)) = cache.iter().find_position(|p| *p == self) {
                break (index, cache.len());
            }
            let current = self.clone();
            cache.push(current);
            self.tilt_rotate();
        };

        let loop_length = end - start;
        let offset = CYCLES - start;
        let index = offset % loop_length + start;

        cache[index].clone()
    }

    fn tilt_rotate(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.tilt_north_tile(x, y);
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.width() {
            for y in (0..self.height()).rev() {
                self.tilt_south_tile(x, y);
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height() {
            for x in (0..self.width()).rev() {
                self.tilt_east_tile(x, y);
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.tilt_west_tile(x, y);
            }
        }
    }

    #[inline]
    fn tilt_north_tile(&mut self, x: usize, mut y: usize) {
        if self.rocks[y][x] != Some(Rock::Round) {
            return;
        }

        loop {
            if y == 0 {
                break;
            }
            y -= 1;
            let next_tile = self.rocks[y][x];
            if next_tile != None {
                break;
            }

            self.rocks[y][x] = Some(Rock::Round);
            self.rocks[y + 1][x] = None;
        }
    }

    #[inline]
    fn tilt_south_tile(&mut self, x: usize, mut y: usize) {
        if self.rocks[y][x] != Some(Rock::Round) {
            return;
        }

        loop {
            if y == self.height() - 1 {
                break;
            }
            y += 1;
            let next_tile = self.rocks[y][x];
            if next_tile != None {
                break;
            }

            self.rocks[y][x] = Some(Rock::Round);
            self.rocks[y - 1][x] = None;
        }
    }

    #[inline]
    fn tilt_east_tile(&mut self, mut x: usize, y: usize) {
        if self.rocks[y][x] != Some(Rock::Round) {
            return;
        }

        loop {
            if x == self.width() - 1 {
                break;
            }
            x += 1;
            let next_tile = self.rocks[y][x];
            if next_tile != None {
                break;
            }

            self.rocks[y][x] = Some(Rock::Round);
            self.rocks[y][x - 1] = None;
        }
    }

    #[inline]
    fn tilt_west_tile(&mut self, mut x: usize, y: usize) {
        if self.rocks[y][x] != Some(Rock::Round) {
            return;
        }

        loop {
            if x == 0 {
                break;
            }
            x -= 1;
            let next_tile = self.rocks[y][x];
            if next_tile != None {
                break;
            }

            self.rocks[y][x] = Some(Rock::Round);
            self.rocks[y][x + 1] = None;
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => Some(Rock::Round),
                        '#' => Some(Rock::Square),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Ok(Self { rocks })
    }
}
