use anyhow::*;
use aoc_2023::*;
use hail::Hail;
use itertools::Itertools;
use line2d::Line2D;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

mod hail;
mod line2d;
mod vec3;

struct Day;

impl BasicSolution for Day {
    type Parsed = Hail;
    type Answer = u64;
    type TestAnswer = Self::Answer;

    const DATA: &'static str = include_str!("input.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_ANSWER_A: Self::TestAnswer = 2;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 47;

    fn part1(input: Self::Parsed) -> Result<Self::Answer> {
        let Hail {
            stones,
            testing_area,
        } = input;
        let lines = stones.into_iter().map(|stone| {
            Line2D::new(
                stone.position.into(),
                (stone.position + stone.velocity).into(),
            )
        });

        Ok(lines
            .clone()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .filter_map(|(a, b)| Some((a, b, a.intersection_point(&b)?)))
            .filter_map(|(a, b, (x, y))| {
                if a.is_point_after_start((x, y)) && b.is_point_after_start((x, y)) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .filter(|(x, y)| testing_area.contains(x) && testing_area.contains(y))
            .count() as u64)
    }

    fn part2(input: Self::Parsed) -> Result<Self::Answer> {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let stones = input.stones;

        let px = Int::new_const(&ctx, "px");
        let py = Int::new_const(&ctx, "py");
        let pz = Int::new_const(&ctx, "pz");
        let vx = Int::new_const(&ctx, "vx");
        let vy = Int::new_const(&ctx, "vy");
        let vz = Int::new_const(&ctx, "vz");

        for stone in stones {
            let px_n = Int::from_i64(&ctx, stone.position.x);
            let py_n = Int::from_i64(&ctx, stone.position.y);
            let pz_n = Int::from_i64(&ctx, stone.position.z);
            let vx_n = Int::from_i64(&ctx, stone.velocity.x);
            let vy_n = Int::from_i64(&ctx, stone.velocity.y);
            let vz_n = Int::from_i64(&ctx, stone.velocity.z);
            let t_n = Int::fresh_const(&ctx, "t");

            solver.assert(&(&px_n + &vx_n * &t_n)._eq(&(&px + &vx * &t_n)));
            solver.assert(&(&py_n + &vy_n * &t_n)._eq(&(&py + &vy * &t_n)));
            solver.assert(&(&pz_n + &vz_n * &t_n)._eq(&(&pz + &vz * &t_n)));
        }
        solver.check();

        let model = solver.get_model().unwrap();
        let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
        let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
        let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

        Ok((x + y + z).try_into()?)
    }

    fn parse(data: &str) -> Result<Self::Parsed> {
        data.parse()
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
