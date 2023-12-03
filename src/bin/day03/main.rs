use anyhow::*;
use aoc_2023::*;

pub mod schematic;
use itertools::Itertools;
use schematic::*;

struct Day;

impl BasicSolution for Day {
    type Parsed = Schematic;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 467835;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input
            .numbers
            .iter()
            .filter(|number| input.is_number_touching_component(number))
            .map(|number| number.n)
            .sum())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let gears = input
            .raw_schematic
            .map
            .iter()
            .filter_map(|(coord, c)| match c {
                '*' => Some(coord),
                _ => None,
            });

        let numbers = gears
            .filter_map(|coords| {
                input
                    .numbers
                    .iter()
                    .filter(|number| number.intersects(coords))
                    .collect_tuple::<(_, _)>()
            })
            .map(|(a, b)| a.n * b.n)
            .sum();

        Ok(numbers)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.try_into()
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
