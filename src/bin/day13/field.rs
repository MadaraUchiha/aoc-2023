use std::str::FromStr;

use anyhow::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Field(Vec<Vec<char>>);

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.lines().map(|line| line.chars().collect()).collect()))
    }
}

impl Field {
    pub fn find_all_reflections<const ALLOWANCE: usize>(&self) -> Option<usize> {
        if let Some(row) = self.find_reflection::<ALLOWANCE>() {
            return Some(row * 100);
        }
        let r = self.rotate();
        if let Some(col) = r.find_reflection::<ALLOWANCE>() {
            return Some(col);
        }

        println!("Failed to find reflection for {:?}", self);

        None
    }
    pub fn find_reflection<const ALLOWNACE: usize>(&self) -> Option<usize> {
        (1..self.0.len()).find(|&i| {
            let first_i_lines = &self.0[..i];
            let last_i_lines = &self.0[i..];

            let pairs = first_i_lines
                .into_iter()
                .rev()
                .zip(last_i_lines.into_iter());

            let diff = pairs
                .map(|(a, b)| {
                    a.into_iter()
                        .zip(b.into_iter())
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>();

            diff == ALLOWNACE
        })
    }

    pub fn rotate(&self) -> Self {
        Self(
            (0..self.0[0].len())
                .map(|i| self.0.iter().map(|row| row[i]).collect_vec())
                .collect_vec(),
        )
    }
}
