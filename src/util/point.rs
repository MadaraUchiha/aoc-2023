use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

pub const ADJACENT: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const ADJACENT_DIAG: [Point; 8] = [
    Point::new(-1, -1),
    UP,
    Point::new(1, -1),
    LEFT,
    RIGHT,
    Point::new(-1, 1),
    DOWN,
    Point::new(1, 1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn new_u32(x: u32, y: u32) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    #[inline]
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    pub fn signum(self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }
}

impl From<u8> for Point {
    fn from(input: u8) -> Self {
        match input {
            b'U' | b'^' => UP,
            b'D' | b'v' => DOWN,
            b'L' | b'<' => LEFT,
            b'R' | b'>' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(input: (i32, i32)) -> Self {
        Self::new(input.0, input.1)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y
    }
}
