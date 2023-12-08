use anyhow::*;
use aoc_2023::*;
use itertools::{process_results, Itertools};
use network::*;
use num::Integer;

struct Day;

mod network;

impl BasicSolution for Day {
    type Parsed = Network;
    type Answer = u64;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample2.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 6;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 6;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        input.steps_to_reach("AAA", |node| node == "ZZZ")
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let nodes = input.nodes.iter().filter(|(id, _)| id.ends_with("A"));

        let visits = nodes.map(|(id, _)| {
            let steps = input.steps_to_reach(id, |node| node.ends_with("Z"))?;
            Ok(steps)
        });

        process_results(visits, |it| it.reduce(|a, b| a.lcm(&b)).unwrap())
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
        let err = || anyhow!("Invalid input");
        let (instruction_str, nodes_str) = data.split_once("\n\n").ok_or_else(err)?;
        let instructions = instruction_str
            .chars()
            .map(|c| c.try_into())
            .try_collect()?;

        let nodes = process_results(nodes_str.lines().map(|line| line.try_into()), |it| {
            it.map(|node: Node| (node.id, node)).collect()
        })?;

        Ok(Network {
            instructions,
            nodes,
        })
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
