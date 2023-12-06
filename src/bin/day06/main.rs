use anyhow::*;
use aoc_2023::*;
use itertools::{process_results, Itertools};

#[derive(Debug, Clone, Default)]
struct Race {
    time: usize,
    distance_to_beat: usize,
}

#[derive(Debug, Clone)]
struct RaceGame {
    individual_races: Vec<Race>,
    big_race: Race,
}

struct Day;

impl BasicSolution for Day {
    type Parsed = RaceGame;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 288;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 71503;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("Part 1 error :(");
        process_results(
            input
                .individual_races
                .into_iter()
                .map(|race| race.winning_times().ok_or_else(err)),
            |it| it.product(),
        )
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let err = || anyhow!("Part 2 error :(");
        let big_race = input.big_race;

        big_race.winning_times().ok_or_else(err)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        let err = || anyhow!("Parse failure :(");
        let (time_str, distance_str) = data.split_once('\n').ok_or_else(err)?;
        let times = time_str
            .strip_prefix("Time: ")
            .ok_or_else(err)?
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        let distances = distance_str
            .strip_prefix("Distance: ")
            .ok_or_else(err)?
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        let joined_time: usize = times.iter().join("").parse()?;
        let joined_distance: usize = distances.iter().join("").parse()?;

        let individual_times: Vec<usize> =
            times.into_iter().map(|time| time.parse()).try_collect()?;
        let individual_distances: Vec<usize> = distances
            .into_iter()
            .map(|distance| distance.parse())
            .try_collect()?;

        let zip = individual_times
            .into_iter()
            .zip(individual_distances.into_iter());

        let races = zip
            .map(|(time, distance_to_beat)| Race {
                time,
                distance_to_beat,
            })
            .collect();

        Ok(RaceGame {
            individual_races: races,
            big_race: Race {
                time: joined_time,
                distance_to_beat: joined_distance,
            },
        })
    }
}

pub fn main() -> anyhow::Result<()> {
    Day::main()
}

impl Race {
    fn minimal_time_to_beat(&self) -> Option<usize> {
        (0..=self.time / 2).find(|hold_time| {
            let travel_time = self.time - hold_time;
            hold_time * travel_time > self.distance_to_beat
        })
    }

    fn winning_times(&self) -> Option<usize> {
        let start = self.minimal_time_to_beat()?;
        let end = self.time - start;

        Some(end - start + 1)
    }
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
