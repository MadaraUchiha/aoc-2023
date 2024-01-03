use anyhow::*;
use itertools::{process_results, Itertools};
use std::{ops::RangeInclusive, str::FromStr};

use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Hailstone {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl FromStr for Hailstone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Invalid input");
        let (pos, vel) = s.split_once(" @ ").ok_or_else(err)?;
        let (x, y, z) = process_results(
            pos.split(", ").map(str::trim).map(|coord| coord.parse()),
            |it| it.collect_tuple().ok_or_else(err),
        )??;
        let (dx, dy, dz) = process_results(
            vel.split(", ").map(str::trim).map(|coord| coord.parse()),
            |it| it.collect_tuple().ok_or_else(err),
        )??;
        let position = Vec3::new(x, y, z);
        let velocity = Vec3::new(dx, dy, dz);
        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone)]
pub struct Hail {
    pub stones: Vec<Hailstone>,
    pub testing_area: RangeInclusive<f64>,
}

impl FromStr for Hail {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Invalid input");
        // Yeah, I've modified the input so that I can test the example
        // So sue me.
        let (area_str, stones_str) = s.split_once("\n\n").ok_or_else(err)?;
        let (from, to) = area_str.split_once("..").ok_or_else(err)?;
        let from = from.parse::<f64>()?;
        let to = to.parse::<f64>()?;
        let testing_area = from..=to;

        let stones = stones_str
            .lines()
            .map(str::trim)
            .map(Hailstone::from_str)
            .try_collect()?;

        Ok(Self {
            stones,
            testing_area,
        })
    }
}
