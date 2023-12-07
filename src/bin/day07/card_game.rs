use anyhow::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        Ok(match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Err(anyhow!("Invalid card: {}", s))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub playing_with_jokers: bool,
    pub cards: [Card; 5],
    pub bid: u32,
}

impl Game {
    pub fn kind(&self) -> HandKind {
        let mut counts = [0; 15];
        let mut joker_count = 0;
        for card in &self.cards {
            counts[*card as usize] += 1;
            if self.playing_with_jokers && *card == Card::Jack {
                joker_count += 1;
            }
        }
        counts.sort();
        counts.reverse();

        let joker_position = counts.iter().position(|&c| c == joker_count);

        if let Some(joker_position) = joker_position {
            if joker_position == 0 {
                counts[1] += joker_count;
                counts[0] = 0;
            } else {
                counts[joker_position] = 0;
                counts[0] += joker_count;
            }
        }

        counts.sort();
        counts.reverse();

        let [a, b, ..] = counts;

        if a == 5 {
            return HandKind::FiveOfAKind;
        }
        if a == 4 {
            return HandKind::FourOfAKind;
        }
        if a == 3 && b == 2 {
            return HandKind::FullHouse;
        }
        if a == 3 {
            return HandKind::ThreeOfAKind;
        }
        if a == 2 && b == 2 {
            return HandKind::TwoPair;
        }
        if a == 2 {
            return HandKind::OnePair;
        }
        return HandKind::HighCard;
    }

    fn cards(&self) -> [Card; 5] {
        let mut result = self.cards.clone();
        for card in &mut result {
            if *card == Card::Jack && self.playing_with_jokers {
                *card = Card::Joker;
            }
        }
        result
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&self.kind(), &self.cards()).partial_cmp(&(&other.kind(), &other.cards()))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.kind(), &self.cards()).cmp(&(&other.kind(), &other.cards()))
    }
}
