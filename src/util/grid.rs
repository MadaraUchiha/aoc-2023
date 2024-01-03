use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
};

use itertools::Itertools;

use crate::util::point::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw = input.lines().map(str::as_bytes).collect_vec();
        let height = raw.len() as i32;
        let width = raw[0].len() as i32;
        let mut data = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|row| data.extend_from_slice(row));

        Self {
            width,
            height,
            data,
        }
    }

    pub fn iter(&self) -> GridIterator<'_, u8> {
        GridIterator {
            grid: self,
            index: Point::new(0, 0),
        }
    }

    pub fn find(&self, value: u8) -> Option<Point> {
        let i = self.data.iter().position(|&v| v == value)?;
        Some(Point::new(i as i32 % self.width, i as i32 / self.width))
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        let len = self.data.len() as i32;
        point.x + point.y * self.width >= 0 && point.x + point.y * self.width < len
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[(index.y * self.width + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[(index.y * self.width + index.x) as usize]
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    index: Point,
}

impl Iterator for GridIterator<'_, u8> {
    type Item = (Point, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.y >= self.grid.height {
            return None;
        }

        let item = Some((self.index, self.grid[self.index]));
        self.index.x += 1;
        if self.index.x >= self.grid.width {
            self.index.x = 0;
            self.index.y += 1;
        }
        item
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x, y)] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
