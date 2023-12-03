use anyhow::*;
use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug, Clone)]
pub struct RawSchematic {
    pub map: HashMap<(u32, u32), char>,
}

impl RawSchematic {
    fn is_component(&self, coords: &(u32, u32)) -> bool {
        if let Some(c) = self.map.get(coords) {
            !c.is_ascii_digit() && *c != '.'
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchematicNumber {
    y: u32,
    x: RangeInclusive<u32>,
    pub n: u32,
}

impl SchematicNumber {
    pub fn adjacent(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        let y_range = self.y.saturating_sub(1)..=self.y + 1;
        let x_range = self.x.start().saturating_sub(1)..=self.x.end() + 1;
        x_range.cartesian_product(y_range)
    }

    pub fn intersects(&self, coords: &(u32, u32)) -> bool {
        self.adjacent().any(|coord| coord == *coords)
    }
}

#[derive(Debug, Clone)]
pub struct Schematic {
    pub raw_schematic: RawSchematic,
    pub numbers: Vec<SchematicNumber>,
}

impl Schematic {
    pub fn is_number_touching_component(&self, number: &SchematicNumber) -> bool {
        number
            .adjacent()
            .any(|coord| self.raw_schematic.is_component(&coord))
    }
}

impl TryFrom<&str> for Schematic {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| ((x as u32, y as u32), c))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<HashMap<_, _>>();

        let raw_schematic = RawSchematic { map };
        let numbers = find_numbers(&raw_schematic)?;
        Ok(Self {
            raw_schematic,
            numbers,
        })
    }
}

fn find_numbers(RawSchematic { map }: &RawSchematic) -> Result<Vec<SchematicNumber>> {
    let err = || anyhow!("Parse Error :(");

    let filtered_schematic = map
        .into_iter()
        .filter(|(_, c)| c.is_ascii_digit())
        .map(|(&(x, y), c)| ((x, y), c.to_digit(10).unwrap()))
        .collect::<HashMap<_, _>>();

    let mut sorted_coordinates = filtered_schematic.keys().collect_vec();
    sorted_coordinates.sort_by(
        |(x1, y1), (x2, y2)| {
            if y1 == y2 {
                x1.cmp(x2)
            } else {
                y1.cmp(y2)
            }
        },
    );

    let mut batches = Vec::new();
    let mut slice_start = 0;
    for i in 1..sorted_coordinates.len() {
        let (x1, y1) = *sorted_coordinates[i - 1];
        let (x2, y2) = *sorted_coordinates[i];

        if y1 != y2 || x1 + 1 != x2 {
            batches.push(&sorted_coordinates[slice_start..i]);
            slice_start = i;
        }
    }
    if sorted_coordinates.len() > 0 {
        batches.push(&sorted_coordinates[slice_start..]);
    }

    let numbers = batches
        .iter()
        .map(|batch| {
            let y = batch[0].1;
            let x = batch.first().unwrap().0..=batch.last().unwrap().0;
            let n = itertools::process_results(
                batch
                    .iter()
                    .map(|coord| map.get(coord).ok_or_else(err)?.to_digit(10).ok_or_else(err)),
                |iter| iter.fold(0, |acc, n| acc * 10 + n),
            )?;
            Ok(SchematicNumber { y, x, n })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(numbers)
}
