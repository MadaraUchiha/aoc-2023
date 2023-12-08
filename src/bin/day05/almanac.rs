use anyhow::*;
use itertools::{process_results, Itertools};
use std::{ops::Range, str::FromStr};

#[derive(Debug, Clone)]
pub struct Almanac {
    pub initial_seeds: Vec<u64>,
    pub range_chain: Vec<RangeBlock>,
}

impl Almanac {
    pub fn seed_ranges(&self) -> impl Iterator<Item = Range<u64>> + '_ {
        self.initial_seeds
            .iter()
            .tuples()
            .map(|(start, length)| *start..*start + *length)
    }
}

#[derive(Debug, Clone)]
pub struct RangeBlock(pub Vec<RangeMapping>);

#[derive(Debug, Clone)]
pub struct RangeMapping {
    range: Range<u64>,
    offset: i64,
}

impl RangeMapping {
    pub fn map(&self, value: u64) -> u64 {
        value
            .checked_add_signed(self.offset)
            .unwrap_or_else(|| panic!("overflow: {} + {}", value, self.offset))
    }

    pub fn map_range(&self, range: Range<u64>) -> ChainedRangeMapping {
        let to_option = |start, end| -> Option<Range<u64>> {
            if (start..end).is_empty() {
                None
            } else {
                Some(start..end)
            }
        };
        let before = to_option(range.start, self.range.start.min(range.end));
        let after = to_option(self.range.end.max(range.start), range.end);
        let overlap = to_option(
            self.range.start.max(range.start),
            self.range.end.min(range.end),
        )
        .map(|r| self.map(r.start)..self.map(r.end));

        ChainedRangeMapping {
            before,
            overlap,
            after,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChainedRangeMapping {
    pub before: Option<Range<u64>>,
    pub overlap: Option<Range<u64>>,
    pub after: Option<Range<u64>>,
}

impl RangeBlock {
    pub fn map(&self, value: u64) -> u64 {
        self.0
            .iter()
            .find_map(|mapping| mapping.range.contains(&value).then(|| mapping.map(value)))
            .unwrap_or(value)
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("invalid input");
        let (seeds_line, rest) = s.split_once("\n\n").ok_or_else(err)?;
        let initial_seeds = seeds_line
            .strip_prefix("seeds: ")
            .ok_or_else(err)?
            .split_ascii_whitespace()
            .map(|line| line.parse().map_err(|_| err()))
            .try_collect()?;

        let range_chain = rest.split("\n\n").map(parse_map_block).try_collect()?;

        Ok({
            Almanac {
                initial_seeds,
                range_chain,
            }
        })
    }
}

fn parse_map_block(block: &str) -> Result<RangeBlock> {
    let err = || anyhow!("invalid input");

    let lines = block.lines().skip(1);

    let parse_range = |line: &str| -> Result<RangeMapping> {
        let (to_start, from_start, length) =
            process_results(line.split_ascii_whitespace().map(u64::from_str), |it| {
                it.collect_tuple().ok_or_else(err)
            })??;

        Ok({
            RangeMapping {
                range: from_start..from_start + length,
                offset: (to_start as i64 - from_start as i64),
            }
        })
    };

    Ok(RangeBlock(lines.map(parse_range).try_collect()?))
}

pub fn map_range_chain(
    RangeBlock(mappings): &RangeBlock,
    range: Range<u64>,
) -> impl Iterator<Item = Range<u64>> + '_ {
    let mut queue = vec![range];
    let mut next_queue = vec![];
    let mut overlaps = vec![];

    for mapping in mappings {
        for range in queue.drain(..) {
            let mapped = mapping.map_range(range);
            next_queue.extend(mapped.before);
            next_queue.extend(mapped.after);
            overlaps.extend(mapped.overlap);
        }
        queue.extend(next_queue.drain(..));
    }
    overlaps.into_iter().chain(queue)
}
