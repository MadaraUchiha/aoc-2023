use almanac::{map_range_chain, Almanac};
use anyhow::*;
use aoc_2023::*;

struct Day;

mod almanac;

impl BasicSolution for Day {
    type Parsed = Almanac;
    type Answer = u64;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 35;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 46;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        input
            .initial_seeds
            .iter()
            .map(|seed| {
                input
                    .range_chain
                    .iter()
                    .fold(*seed, |value, block| block.map(value))
            })
            .min()
            .ok_or_else(|| anyhow!("no seeds"))
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let ranges = input.seed_ranges();

        let all_possible_ranges = ranges.flat_map(|range| {
            input.range_chain.iter().fold(vec![range], |acc, block| {
                acc.into_iter()
                    .flat_map(|range| map_range_chain(block, range))
                    .collect()
            })
        });

        all_possible_ranges
            .map(|r| r.start)
            .min()
            .ok_or_else(|| anyhow!("empty seeds"))
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.parse()
    }
}

pub fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
