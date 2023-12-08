use anyhow::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    Left,
    Right,
}
impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self> {
        match s {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(anyhow!("Invalid instruction: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: &'static str,
    pub left: &'static str,
    pub right: &'static str,
}

impl TryFrom<&'static str> for Node {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self> {
        let err = || anyhow!("Invalid node");
        let (id, left_right) = s.split_once(" = ").ok_or_else(err)?;
        let (left, right) = left_right
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(", "))
            .ok_or_else(err)?;

        Ok(Node { id, left, right })
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    pub nodes: HashMap<&'static str, Node>,
    pub instructions: Vec<Instruction>,
}

impl Network {
    pub fn steps_to_reach<P>(&self, start: &'static str, predicate: P) -> Result<u64>
    where
        P: Fn(&str) -> bool,
    {
        let err = || anyhow!("Failed to find element");
        let mut steps_taken = 0u32;
        let mut current_node = start;

        for instruction in self.instructions.iter().cycle() {
            let node = self.nodes.get(current_node).ok_or_else(err)?;
            let next_node = match instruction {
                Instruction::Left => node.left,
                Instruction::Right => node.right,
            };

            steps_taken += 1;

            if predicate(next_node) {
                break;
            }

            current_node = next_node;
        }

        Ok(steps_taken.into())
    }
}
