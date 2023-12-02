use anyhow::*;
use aoc_2023::*;

struct Day;

impl BasicSolution for Day {
    type Parsed = Vec<Game>;
    type Answer = u32;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2286;

    fn part1(input: Self::Parsed) -> Result<u32> {
        const TARGET: Revealed = Revealed {
            red: 12,
            green: 13,
            blue: 14,
        };
        let is_good_game = |game: &Game| {
            let revealed = &game.revealed;
            revealed.red <= TARGET.red
                && revealed.blue <= TARGET.blue
                && revealed.green <= TARGET.green
        };
        Ok(input
            .into_iter()
            .filter(is_good_game)
            .map(|game| game.id)
            .sum())
    }

    fn part2(input: Self::Parsed) -> Result<u32> {
        let power = |game: Game| {
            let revealed = &game.revealed;
            revealed.red as u32 * revealed.blue as u32 * revealed.green as u32
        };
        Ok(input.into_iter().map(power).sum())
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.lines().into_iter().map(parse_game).collect()
    }
}

fn parse_game(input: &str) -> Result<Game> {
    let (id_str, revealed_str) = input
        .split_once(": ")
        .ok_or_else(|| anyhow!("Invalid input"))?;
    let id = id_str
        .strip_prefix("Game ")
        .ok_or_else(|| anyhow!("Invalid input"))?
        .parse()?;
    let revealed = revealed(revealed_str)?;
    Ok(Game { id, revealed })
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    id: u32,
    revealed: Revealed,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Revealed {
    red: u8,
    blue: u8,
    green: u8,
}

fn revealed(input: &str) -> Result<Revealed> {
    let rounds = input.split("; ");

    let update_max = |revealed: &mut Revealed, round_description: &str| {
        let colors = round_description.split(", ");
        for color in colors {
            let (count_str, color) = color
                .split_once(" ")
                .ok_or_else(|| anyhow!("Invalid input"))?;
            let count = count_str.parse()?;
            match color {
                "red" => revealed.red = revealed.red.max(count),
                "blue" => revealed.blue = revealed.blue.max(count),
                "green" => revealed.green = revealed.green.max(count),
                _ => return Err(anyhow!("Invalid input")),
            }
        }
        Ok(revealed.clone())
    };

    rounds.fold(Ok(Revealed::default()), |acc, round| {
        acc.and_then(|mut revealed| update_max(&mut revealed, round))
    })
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
