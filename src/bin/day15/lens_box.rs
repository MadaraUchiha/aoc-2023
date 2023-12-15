use anyhow::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensBox {
    boxes: Vec<Vec<Lens>>,
}

impl LensBox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn focus_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.value as usize)
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn run_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Insert(address, lens) => {
                    self.insert(*address, *lens);
                }
                Instruction::Remove(address, label) => {
                    self.remove(*address, *label);
                }
            }
        }
    }

    fn insert(&mut self, address: u8, lens: Lens) {
        let box_index = address as usize;
        let position = self.boxes[box_index]
            .iter()
            .position(|l| l.label == lens.label);

        match position {
            Some(position) => {
                self.boxes[box_index][position] = lens;
            }
            None => {
                self.boxes[box_index].push(lens);
            }
        }
    }

    fn remove(&mut self, address: u8, label: &'static str) {
        let box_index = address as usize;
        let position = self.boxes[box_index].iter().position(|l| l.label == label);

        if let Some(position) = position {
            self.boxes[box_index].remove(position);
        }
    }
}

impl Default for LensBox {
    fn default() -> Self {
        Self {
            boxes: vec![vec![]; 256],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lens {
    value: u8,
    label: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Insert(u8, Lens),
    Remove(u8, &'static str),
}

impl TryFrom<&'static str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = || anyhow!("Invalid instruction: {}", s);
        let is_insert = s.contains('=');
        Ok(if is_insert {
            let (label, value) = s.split_once('=').ok_or_else(err)?;
            let address = hash(label);
            let value = value.parse().map_err(|_| err())?;
            Instruction::Insert(address, Lens { value, label })
        } else {
            let label = s[..s.len() - 1].trim();
            let address = hash(label);
            Instruction::Remove(address, label)
        })
    }
}

pub fn hash(str: &str) -> u8 {
    let mut result: u8 = 0;
    for c in str.chars() {
        result = result.wrapping_add(c as u8);
        result = result.wrapping_mul(17);
    }
    // println!("{} -> {}", str, result);
    result
}
