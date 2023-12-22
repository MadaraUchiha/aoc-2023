use std::collections::HashMap;

use anyhow::*;
use aoc_2023::*;
use modules::*;
use num::Integer;

mod modules;

struct Day;

impl BasicSolution for Day {
    type Parsed = Circuit;
    type Answer = usize;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 11687500;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 1;

    fn part1(mut input: Self::Parsed) -> Result<Self::Answer> {
        let mut high = 0;
        let mut low = 0;

        for _ in 0..1000 {
            let CircuitResult {
                high: cycle_high,
                low: cycle_low,
                circuit,
            } = input.press_the_button(&mut HashMap::new());
            high += cycle_high;
            low += cycle_low;
            input = circuit;
        }

        Ok(high * low)
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let module_to_track = input
            .modules
            .values()
            .find_map(|module| module.destinations.contains(&"rx").then_some(module.name))
            .expect("Module pointing to rx not found");

        let Some(Module {
            module_type: ModuleType::Conjunction(inputs),
            ..
        }) = input.modules.get(module_to_track)
        else {
            bail!("Expected module pointing to rx to be a conjunction")
        };

        let mut minimal_presses: HashMap<&&str, Option<usize>> = inputs
            .into_iter()
            .map(|(name, _)| (name, None))
            .collect::<HashMap<_, _>>();

        let mut circuit = input.clone();

        for press in 1..10_000 {
            let mut got_high_press = minimal_presses.iter().map(|(&&k, _)| (k, false)).collect();

            let CircuitResult {
                circuit: next_circuit,
                ..
            } = circuit.press_the_button(&mut got_high_press);

            for (name, signal) in got_high_press.iter() {
                if !signal {
                    continue;
                }
                let minimal_press = minimal_presses
                    .get_mut(name)
                    .expect("Module not found in minimal presses");

                *minimal_press = Some(press);

                if minimal_presses.values().all(|value| value.is_some()) {
                    return Ok(minimal_presses
                        .values()
                        .map(|v| v.unwrap())
                        .fold(1, |a, b| a.lcm(&b)));
                }
            }

            circuit = next_circuit;
        }

        bail!("Failed to find minimal presses after 10k tries")
    }

    fn parse(data: &'static str) -> Result<Self::Parsed> {
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
