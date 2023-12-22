use anyhow::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circuit {
    pub modules: HashMap<&'static str, Module>,
}

impl Circuit {
    pub fn press_the_button(&self, track: &mut HashMap<&'static str, bool>) -> CircuitResult {
        let mut circuit = self.clone();
        let mut high = 0;
        let mut low = 1; // start at 1 because the button is always low
        let mut queue = VecDeque::from(vec![(
            "broadcast",
            Pulse {
                value: Low,
                from: "button",
            },
        )]);

        while let Some((name, pulse)) = queue.pop_front() {
            let module = circuit
                .modules
                .get_mut(name)
                .expect(format!("No module named {}", name).as_str());
            let signal = module.module_type.process(pulse);
            if let None = signal {
                continue;
            }
            let signal = signal.unwrap();
            if track.contains_key(name) && signal == High {
                track.insert(name, true);
            }
            for to in module.destinations.iter() {
                match signal {
                    High => {
                        high += 1;
                    }
                    Low => low += 1,
                }
                queue.push_back((
                    *to,
                    Pulse {
                        value: signal,
                        from: name,
                    },
                ));
            }
        }

        CircuitResult { circuit, high, low }
    }
}

pub struct CircuitResult {
    pub circuit: Circuit,
    pub high: usize,
    pub low: usize,
}

impl TryFrom<&'static str> for Circuit {
    type Error = anyhow::Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let mut modules_without_conjunction_dests = HashMap::new();

        for line in value.lines() {
            let module: Module = line.try_into()?;
            modules_without_conjunction_dests.insert(module.name, module);
        }

        let mut modules = modules_without_conjunction_dests.clone();

        // Second pass to add inputs to all of the conjunctions
        for name in modules_without_conjunction_dests.keys() {
            if let Conjunction(_) = modules_without_conjunction_dests[name].module_type {
                let sending_names = modules_without_conjunction_dests
                    .values()
                    .filter(move |Module { destinations, .. }| destinations.contains(&name))
                    .map(|Module { name, .. }| *name);

                let new_inputs = sending_names.map(|name| (name, Low)).collect::<Vec<_>>();

                modules.insert(
                    name,
                    Module {
                        module_type: Conjunction(new_inputs),
                        ..modules_without_conjunction_dests[name].clone()
                    },
                );
            }
        }

        let all_known_names = modules_without_conjunction_dests
            .values()
            .flat_map(|module| module.destinations.iter());

        // Third pass to add null modules for all of the names that are destinations but not sources
        for name in all_known_names {
            if !modules.contains_key(name) {
                modules.insert(
                    name,
                    Module {
                        name,
                        module_type: Null,
                        destinations: vec![],
                    },
                );
            }
        }

        Ok(Self { modules })
    }
}

struct Pulse {
    value: PulseValue,
    from: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseValue {
    High,
    Low,
}
use PulseValue::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    FlipFlop(bool),
    Conjunction(Vec<(&'static str, PulseValue)>),
    Broadcast,
    Null,
}

use ModuleType::*;

impl ModuleType {
    fn process(&mut self, Pulse { value, from }: Pulse) -> Option<PulseValue> {
        match self {
            FlipFlop(ref mut state) => match value {
                High => None,
                Low => {
                    *state = !*state;

                    match *state {
                        true => Some(High),
                        false => Some(Low),
                    }
                }
            },
            Conjunction(ref mut inputs) => {
                let (_, ref mut input_state) = inputs.iter_mut().find(|(name, _)| *name == from)?;
                *input_state = value;
                let all_high = inputs.iter().all(|(_, state)| *state == High);

                match all_high {
                    true => Some(Low),
                    false => Some(High),
                }
            }
            Broadcast => Some(value),
            Null => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: &'static str,
    pub module_type: ModuleType,
    pub destinations: Vec<&'static str>,
}

impl TryFrom<&'static str> for Module {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = |s| anyhow!("Invalid module: {}", s);
        let (type_name, destinations) = s.split_once(" -> ").ok_or_else(|| err(s))?;
        let destinations = destinations.split(", ").collect::<Vec<_>>();

        let (module_type, name) = match type_name {
            "broadcaster" => (Broadcast, "broadcast"),
            s if &s[0..1] == "%" => (FlipFlop(false), &s[1..]),
            s if &s[0..1] == "&" => (Conjunction(vec![]), &s[1..]),
            s => (Null, s),
        };

        Ok(Self {
            name,
            module_type,
            destinations,
        })
    }
}
