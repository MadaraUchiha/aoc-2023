use anyhow::{anyhow, Result};
use aoc_2023::*;
use std::str::Lines;

struct Day;

impl BasicSolution for Day {
    type Parsed = Lines<'static>;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample1.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample2.txt");

    const SAMPLE_ANSWER_A: Self::TestAnswer = 142;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 281;

    fn part1(input: Self::Parsed) -> Result<u32> {
        solve(input, &[])
    }

    fn part2(input: Self::Parsed) -> Result<u32> {
        solve(
            input,
            &[
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ],
        )
    }

    fn parse(input: &'static str) -> IResult<Self::Parsed> {
        Ok(input.lines())
    }
}

fn solve(lines: Lines, digit_words: &[(&str, u32)]) -> Result<u32> {
    let line_to_calibration_value = |line: &str| {
        let err = || anyhow!("Couldn't find a digit in line '{line}'");

        let digit_at_i = |i| {
            let digit = line[i..i + 1].parse().ok();
            let match_word = |&(digit, val)| line[i..].starts_with(digit).then_some(val);
            digit.or_else(|| digit_words.iter().find_map(match_word))
        };

        let first = (0..line.len()).find_map(digit_at_i).ok_or_else(err)?;
        let last = (0..line.len()).rev().find_map(digit_at_i).ok_or_else(err)?;

        Ok(first * 10 + last)
    };
    lines.map(line_to_calibration_value).sum()
}

pub fn main() -> Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> Result<()> {
        Day::test_b()
    }
}
