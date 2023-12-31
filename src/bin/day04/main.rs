use anyhow::*;
use aoc_2023::*;
use card::{Card, Cards};

struct Day;

mod card;

impl BasicSolution for Day {
    type Parsed = Cards;
    type Answer = u32;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 13;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 30;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        Ok(input.0.iter().map(Card::card_score).sum::<u32>())
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        // 1 of each card we start with
        let mut cards_won = vec![1; input.0.len()];

        for (idx, card) in input.0.iter().enumerate() {
            let this_card_winnings = card.number_of_winning_cards();
            let id: usize = idx + 1;
            // cards starting from next limit by card winning
            for winning_idx in id..id + this_card_winnings {
                // increase by number of copies of this card we've won so far
                cards_won[winning_idx as usize] += cards_won[idx];
            }
        }

        Ok(cards_won.iter().sum())
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
