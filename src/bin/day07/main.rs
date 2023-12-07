use anyhow::*;
use aoc_2023::*;
use card_game::*;
use itertools::process_results;
use itertools::Itertools;

struct Day;

mod card_game;

impl BasicSolution for Day {
    type Parsed = Vec<Game>;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 6440;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 5905;

    fn part1(mut input: Self::Parsed) -> Result<Self::Answer> {
        input.sort();

        Ok(play(input))
    }

    fn part2(mut input: Self::Parsed) -> Result<Self::Answer> {
        for card in input.iter_mut() {
            card.playing_with_jokers = true;
        }
        input.sort();

        Ok(play(input))
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        let err = || anyhow!("Invalid card");
        process_results(
            data.lines()
                .map(|line| line.split_once(" "))
                .map(|pair| pair.ok_or_else(err)),
            |it| {
                it.map(|line| {
                    let (cards_str, bid) = line;
                    let mut cards = [Card::Ace; 5];
                    for (i, c) in cards_str.chars().enumerate() {
                        cards[i] = c.try_into()?;
                    }

                    let bid = bid.parse()?;
                    Ok(Game {
                        cards,
                        bid,
                        playing_with_jokers: false,
                    })
                })
                .try_collect()
            },
        )?
    }
}

fn play(input: Vec<Game>) -> u32 {
    input
        .into_iter()
        .enumerate()
        .map(|(i, game)| {
            let bid = game.bid;
            let score = bid * (i + 1) as u32;
            score
        })
        .sum()
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
