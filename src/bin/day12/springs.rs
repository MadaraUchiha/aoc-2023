use anyhow::*;
use itertools::Itertools;
use std::{collections::HashMap, iter::once, str::FromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpringCondition {
    Operational,
    Broken,
    Unknown,
}
use SpringCondition::*;

impl TryFrom<char> for SpringCondition {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Broken),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("Invalid spring condition: {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpringFormation {
    condition_sequence: Vec<SpringCondition>,
    damaged_groups: Vec<usize>,
}

impl SpringFormation {
    pub fn count_arrangements(mut self) -> usize {
        // Add a trailing operational for the ending condition
        self.condition_sequence.push(Operational);
        let mut cache = HashMap::new();

        count_arrangements_recursive(&self.condition_sequence, &self.damaged_groups, &mut cache)
    }

    pub fn unfold(&self) -> Self {
        let new_condition_sequence = self
            .condition_sequence
            .iter()
            .copied()
            .chain(once(Unknown))
            .cycle()
            .take(5 * self.condition_sequence.len() + 4)
            .collect_vec();

        let new_damaged_groups = self
            .damaged_groups
            .iter()
            .copied()
            .cycle()
            .take(5 * self.damaged_groups.len())
            .collect_vec();

        Self {
            condition_sequence: new_condition_sequence,
            damaged_groups: new_damaged_groups,
        }
    }
}

impl FromStr for SpringFormation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || anyhow!("Invalid spring formation: {}", s);

        let (sequence, groups) = s.split_once(' ').ok_or_else(err)?;

        let condition_sequence = sequence.chars().map(|c| c.try_into()).try_collect()?;
        let damaged_groups = groups.split(',').map(|s| s.parse()).try_collect()?;

        Ok(Self {
            condition_sequence,
            damaged_groups,
        })
    }
}

fn count_arrangements_recursive(
    condition_sequence: &[SpringCondition],
    damaged_groups: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let c_i = condition_sequence.len();
    let g_i = damaged_groups.len();

    let cache_key = (c_i, g_i);

    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let remaining_damaged: usize = damaged_groups.iter().sum();
    let total_remaining_space = remaining_damaged + damaged_groups.len();

    // Ran out of space
    if condition_sequence.len() < total_remaining_space {
        cache.insert(cache_key, 0);
        return 0;
    }

    // Ran out of groups, leftovers?
    if damaged_groups.is_empty() {
        // We still have brokens but no more space for them, invalid arrangement
        if condition_sequence.contains(&Broken) {
            cache.insert(cache_key, 0);
            return 0;
        }

        // Rest of the tiles aren't broken, they must be operational
        cache.insert(cache_key, 1);
        return 1;
    }

    let mut arrangements = 0;
    if condition_sequence[0] != Broken {
        // We can put an operational tile here
        // Try until you run out of tiles
        arrangements +=
            count_arrangements_recursive(&condition_sequence[1..], &damaged_groups, cache);
    }
    let next_group = damaged_groups[0];
    if !condition_sequence[0..next_group].contains(&Operational)
        && condition_sequence[next_group] != Broken
    {
        // We found our broken sequence, move on to the next group, leaving one space between groups
        arrangements += count_arrangements_recursive(
            &condition_sequence[(next_group + 1)..],
            &damaged_groups[1..],
            cache,
        );
    }

    cache.insert(cache_key, arrangements);
    arrangements
}
