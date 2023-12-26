use std::{
    ops::{Add, Neg, Sub},
    str::FromStr,
};

use anyhow::*;
use itertools::{process_results, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl FromStr for V3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("invalid vector: {}", s);
        let (x, y, z) = process_results(s.split(',').map(|s| s.parse()), |it| {
            it.collect_tuple().ok_or_else(err)
        })??;
        Ok(Self { x, y, z })
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
