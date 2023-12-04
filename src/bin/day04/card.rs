use std::num::ParseIntError;

use anyhow::*;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u8,
    pub winning_numbers: Vec<u8>,
    pub actual_numbers: Vec<u8>,
}

impl Card {
    pub fn number_of_winning_cards(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|n| self.actual_numbers.contains(n))
            .count()
    }

    pub fn card_score(&self) -> u32 {
        match self.number_of_winning_cards() {
            0 => 0,
            // Look at me ma, I know bitwise operators!
            n => 1 << (n - 1),
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let err = || anyhow!("Parse Error :(");
        let (prefix, numbers) = value.split_once(": ").ok_or_else(err)?;
        let id = prefix
            .split_ascii_whitespace()
            .last()
            .ok_or_else(err)?
            .parse::<u8>()?;
        let (winning, actual) = numbers.split_once(" | ").ok_or_else(err)?;
        let winning_numbers = parse_numbers(winning)?;
        let actual_numbers = parse_numbers(actual)?;

        Ok(Self {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cards(pub Vec<Card>);

impl TryFrom<&str> for Cards {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Cards(
            value.lines().map(Card::try_from).collect::<Result<_>>()?,
        ))
    }
}

fn parse_numbers(s: &str) -> Result<Vec<u8>> {
    s.split_ascii_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, ParseIntError>>()
        .map_err(|_| anyhow!("Parse Error :("))
}
