use std::str::FromStr;

use anyhow::*;

#[derive(Debug, Clone)]
pub struct Card<'a> {
    pub id: u8,
    pub winning_numbers: Vec<&'a str>,
    pub actual_numbers: Vec<&'a str>,
}

impl<'a> Card<'a> {
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

impl<'a> TryFrom<&'a str> for Card<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self> {
        let err = || anyhow!("Parse Error :(");
        let (prefix, numbers) = value.split_once(": ").ok_or_else(err)?;
        let id = prefix
            .split_ascii_whitespace()
            .last()
            .ok_or_else(err)?
            .parse::<u8>()?;
        let (winning, actual) = numbers.split_once(" | ").ok_or_else(err)?;
        let winning_numbers = parse_numbers(winning);
        let actual_numbers = parse_numbers(actual);

        Ok(Self {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cards<'a>(pub Vec<Card<'a>>);

impl<'a> TryFrom<&'a str> for Cards<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Ok(Cards(
            value.lines().map(Card::try_from).collect::<Result<_>>()?,
        ))
    }
}

fn parse_numbers<'a>(s: &'a str) -> Vec<&'a str> {
    s.split_ascii_whitespace().collect()
}
