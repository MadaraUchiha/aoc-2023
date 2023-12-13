use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Universe {
    pub galaxies: Vec<(u64, u64)>,
}

impl Universe {
    pub fn empty_rows_between(&self, a: &(u64, u64), b: &(u64, u64)) -> u64 {
        let from = a.1.min(b.1);
        let to = a.1.max(b.1);

        (from..to)
            .into_iter()
            .filter(|y| !self.galaxies.iter().any(|(_, gy)| gy == y))
            .count() as u64
    }

    pub fn empty_cols_between(&self, a: &(u64, u64), b: &(u64, u64)) -> u64 {
        let from = a.0.min(b.0);
        let to = a.0.max(b.0);

        (from..to)
            .into_iter()
            .filter(|x| !self.galaxies.iter().any(|(gx, _)| gx == x))
            .count() as u64
    }

    pub fn distance(&self, a: &(u64, u64), b: &(u64, u64), expansion_factor: u64) -> u64 {
        let (ax, ay) = *a;
        let (bx, by) = *b;

        let dx = (ax as i32 - bx as i32).abs() as u64;
        let dy = (ay as i32 - by as i32).abs() as u64;

        let x_expansion = self.empty_cols_between(a, b) * (expansion_factor - 1);
        let y_expansion = self.empty_rows_between(a, b) * (expansion_factor - 1);

        dx + dy + x_expansion + y_expansion
    }
}

impl FromStr for Universe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as u64, y as u64))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Universe { galaxies })
    }
}
