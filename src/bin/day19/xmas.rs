use anyhow::*;
use itertools::{process_results, Itertools};
use std::{array, collections::HashMap, ops::RangeInclusive};

const INITIAL_WORKFLOW: &'static str = "in";
const MINIMAL_VALUE: usize = 1;
const MAXIMAL_VALUE: usize = 4000;

#[derive(Debug, Clone)]
pub struct XMAS {
    parts: Vec<Part>,
    workflows: HashMap<&'static str, Workflow>,
}
impl XMAS {
    pub fn run(&self) -> Option<usize> {
        self.parts.iter().map(|part| self.run_part(part)).sum()
    }

    pub fn count_accepted_ranges(&self) -> usize {
        self.count_accepted_ranges_recursive(&Destination::Workflow("in"), PartRange::new())
    }

    fn count_accepted_ranges_recursive(
        &self,
        destination: &Destination,
        mut ranges: PartRange,
    ) -> usize {
        if !ranges.valid() {
            return 0;
        }

        let mut total = 0;

        let workflow = match destination {
            Workflow(label) => self.workflows.get(label).unwrap(),
            Accepted => return ranges.value(),
            Rejected => return 0,
        };

        for rule in &workflow.rules {
            let (pass, next) = ranges.split(&rule.condition);
            total += self.count_accepted_ranges_recursive(&rule.destination, pass);
            ranges = next;
        }

        total
    }

    fn run_part(&self, part: &Part) -> Option<usize> {
        let mut current_workflow = INITIAL_WORKFLOW;

        while let Some(workflow) = self.workflows.get(current_workflow) {
            let rule = workflow
                .rules
                .iter()
                .find(|rule| part.matches(&rule.condition))?;

            match rule.destination {
                Destination::Accepted => return Some(part.value()),
                Destination::Rejected => return Some(0),
                Destination::Workflow(label) => current_workflow = label,
            }
        }

        None
    }
}
impl TryFrom<&'static str> for XMAS {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = || anyhow!("invalid input");
        let (workflows, parts) = s.split_once("\n\n").ok_or_else(err)?;

        let parts = parts.lines().map(|line| line.try_into()).try_collect()?;
        let workflows = process_results(workflows.lines().map(|line| line.try_into()), |it| {
            it.map(|workflow: Workflow| (workflow.label, workflow))
                .collect()
        })?;

        Ok(Self { parts, workflows })
    }
}

#[derive(Debug, Clone)]
struct Part([usize; 4]);

impl TryFrom<&'static str> for Part {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = || anyhow!("invalid input");
        let without_curlies = &s[1..s.len() - 1];

        let (x, m, a, s) = process_results(
            without_curlies.split(',').map(|part| {
                let value = &part[2..];
                value.parse().map_err(|_| err())
            }),
            |it| it.collect_tuple(),
        )?
        .ok_or_else(err)?;
        Ok(Self([x, m, a, s]))
    }
}

impl Part {
    pub fn matches(&self, condition: &Condition) -> bool {
        match *condition {
            Gt(p, value) => self.0[p as usize] > value,
            Lt(p, value) => self.0[p as usize] < value,
            Always => true,
        }
    }

    pub fn value(&self) -> usize {
        self.0.iter().sum()
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    label: &'static str,
    rules: Vec<Rule>,
}

impl TryFrom<&'static str> for Workflow {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = || anyhow!("invalid input");
        let (label, rules) = s.split_once("{").ok_or_else(err)?;

        let rules = &rules[..rules.len() - 1];

        let rules = rules.split(",").map(|rule| rule.try_into()).try_collect()?;

        Ok(Self { label, rules })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Condition,
    destination: Destination,
}

impl TryFrom<&'static str> for Rule {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> std::prelude::v1::Result<Self, Self::Error> {
        let err = || anyhow!("invalid input");
        match s.split_once(":") {
            None => Ok(Self {
                condition: Always,
                destination: match s {
                    "A" => Destination::Accepted,
                    "R" => Destination::Rejected,
                    label => Destination::Workflow(label),
                },
            }),
            Some((condition, destination)) => {
                let property = match &condition[..1] {
                    "x" => Property::X,
                    "m" => Property::M,
                    "a" => Property::A,
                    "s" => Property::S,
                    _ => return Err(err()),
                };
                let value = condition[2..].parse().map_err(|_| err())?;
                let condition = match &condition[1..2] {
                    ">" => Gt(property, value),
                    "<" => Lt(property, value),
                    _ => return Err(err()),
                };

                let destination = match destination {
                    "A" => Destination::Accepted,
                    "R" => Destination::Rejected,
                    label => Destination::Workflow(label),
                };

                Ok(Self {
                    condition,
                    destination,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Condition {
    Gt(Property, usize),
    Lt(Property, usize),
    Always,
}

use Condition::*;

#[derive(Debug, Clone)]
enum Destination {
    Workflow(&'static str),
    Accepted,
    Rejected,
}
use Destination::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange([RangeInclusive<usize>; 4]);
impl PartRange {
    fn new() -> Self {
        Self(array::from_fn(|_| MINIMAL_VALUE..=MAXIMAL_VALUE))
    }

    fn value(&self) -> usize {
        self.0
            .iter()
            .map(|range| range.try_len().unwrap_or(0))
            .product()
    }

    fn valid(&self) -> bool {
        self.0.iter().all(|range| !range.is_empty())
    }

    fn split(&self, condition: &Condition) -> (PartRange, PartRange) {
        let mut pass = self.clone();
        let mut fail = self.clone();

        match condition {
            Gt(property, value) => {
                let p = *property as usize;
                pass.0[p] = *pass.0[p].start().max(&(value + 1))..=*pass.0[p].end();
                fail.0[p] = *fail.0[p].start()..=*fail.0[p].end().min(value);
            }
            Lt(property, value) => {
                let p = *property as usize;
                pass.0[p] = *pass.0[p].start()..=*pass.0[p].end().min(&(value - 1));
                fail.0[p] = *fail.0[p].start().max(value)..=*fail.0[p].end();
            }
            Always => {
                fail = Self(array::from_fn(|_| 1..=0));
            }
        }

        (pass, fail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_1() {
        let range = PartRange::new();
        let (pass, fail) = range.split(&Gt(Property::X, 2000));
        assert_eq!(pass.0[0], 2001..=MAXIMAL_VALUE);
        assert_eq!(fail.0[0], MINIMAL_VALUE..=2000);
    }

    #[test]
    fn range_2() {
        let range = PartRange::new();
        let (pass1, fail1) = range.split(&Lt(Property::X, 2000));
        let (pass2, fail2) = fail1.split(&Gt(Property::X, 1000));

        assert_eq!(pass1.0[0], MINIMAL_VALUE..=1999);
        assert_eq!(fail1.0[0], 2000..=MAXIMAL_VALUE);
        assert_eq!(pass2.0[0], fail1.0[0]);
        assert_eq!(fail2.valid(), false);
    }

    #[test]
    fn range_3() {
        let mut range = PartRange::new();
        range.0[0] = 1000..=2000;

        let (pass, fail) = range.split(&Always);

        assert_eq!(pass, range);
        assert!(!fail.valid());
    }
}
