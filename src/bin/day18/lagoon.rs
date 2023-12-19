use anyhow::*;
use itertools::Itertools;

#[derive(Debug)]
pub struct Lagoon {
    map: Vec<(isize, isize)>,
}

impl Lagoon {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }
    pub fn dig_trench(&mut self, instructions: Vec<Instruction>) -> &mut Self {
        let mut x: isize = 0;
        let mut y: isize = 0;

        for Instruction {
            direction,
            distance: amount,
            ..
        } in instructions
        {
            let (dx, dy) = match direction {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!("Invalid direction"),
            };

            for _ in 0..amount {
                x += dx;
                y += dy;

                self.map.push((x, y));
            }
        }

        self
    }

    pub fn count(&self) -> Result<usize> {
        let inner_area = shoelace_formula(&self.map);
        let trench_size = self.map.len();

        Ok(trench_size + inner_area + 1 - (trench_size / 2))
    }
}

impl TryFrom<&'static str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        let err = || anyhow!("Invalid input");

        let (direction, amount, color) =
            s.split_ascii_whitespace().collect_tuple().ok_or_else(err)?;

        let amount = amount.parse::<usize>()?;

        let color = &color[2..color.len() - 1];

        Ok(Instruction {
            direction,
            distance: amount,
            color,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    direction: &'static str,
    distance: usize,
    color: &'static str,
}

impl Instruction {
    pub fn into_correct_instruction(&self) -> Result<Instruction> {
        let amount = usize::from_str_radix(&self.color[0..5], 16)?;
        let direction = &self.color[5..];

        let direction = match direction {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("Invalid direction"),
        };

        Ok(Instruction {
            direction,
            distance: amount,
            color: self.color,
        })
    }
}

fn shoelace_formula(loop_coords: &[(isize, isize)]) -> usize {
    loop_coords
        .iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<isize>()
        .unsigned_abs()
        / 2
}
