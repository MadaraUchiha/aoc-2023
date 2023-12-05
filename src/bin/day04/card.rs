use std::str::FromStr;

use anyhow::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Card {
    pub winners: usize,
}

impl Card {
    pub fn number_of_winning_cards(&self) -> usize {
        self.winners as usize
    }

    pub fn card_score(&self) -> u32 {
        match self.winners {
            0 => 0,
            // Look at me ma, I know bitwise operators!
            n => 1 << (n - 1),
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let err = || anyhow!("Parse Error :(");
        let (_, numbers) = value.split_once(": ").ok_or_else(err)?;
        let (winning, owned) = numbers.split_once(" | ").ok_or_else(err)?;
        let winners = winning
            .split_ascii_whitespace()
            .filter(|n| owned.split_ascii_whitespace().contains(n))
            .count();

        Ok(Self { winners })
    }
}

#[derive(Debug, Clone)]
pub struct Cards(pub Vec<Card>);

impl FromStr for Cards {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        Ok(Cards(
            value.lines().map(Card::from_str).collect::<Result<_>>()?,
        ))
    }
}
