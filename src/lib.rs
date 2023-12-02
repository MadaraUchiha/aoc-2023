#![feature(associated_type_defaults)]
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    time::Instant,
};
pub type IResult<T> = Result<T>;

pub trait BasicSolution {
    type Parsed: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const DATA: &'static str;
    const SAMPLE_DATA: &'static str;
    const SAMPLE_DATA_B: &'static str = Self::SAMPLE_DATA;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn part1(data: Self::Parsed) -> Result<Self::Answer>;
    fn part2(data: Self::Parsed) -> Result<Self::Answer>;
}

impl<T: BasicSolution> Solution for T {
    type Parsed = <Self as BasicSolution>::Parsed;
    type ParsedTest = Self::Parsed;
    type Answer = <Self as BasicSolution>::Answer;
    type TestAnswer = <Self as BasicSolution>::TestAnswer;
    const DATA: &'static str = <Self as BasicSolution>::DATA;
    const SAMPLE_DATA: &'static str = <Self as BasicSolution>::SAMPLE_DATA;
    const SAMPLE_DATA_B: &'static str = <Self as BasicSolution>::SAMPLE_DATA_B;
    const SAMPLE_ANSWER_A: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_A;
    const SAMPLE_ANSWER_B: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_B;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        <Self as BasicSolution>::parse(data)
    }

    fn part1(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part1(data)
    }

    fn part2(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part2(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }
    fn a_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer> {
        Self::part1(data)
    }
    fn b_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer> {
        Self::part2(data)
    }
}

pub trait Solution {
    type Parsed: Debug + Clone = &'static str;
    type ParsedTest: Debug + Clone = Self::Parsed;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const DATA: &'static str;
    const SAMPLE_DATA: &'static str;
    const SAMPLE_DATA_B: &'static str = Self::SAMPLE_DATA;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn part1(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn part2(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest>;
    fn a_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer>;
    fn b_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer>;

    fn final_parse(data: &'static str) -> Result<Self::Parsed> {
        Self::parse(data)
    }

    fn final_parse_test(data: &'static str) -> Result<Self::ParsedTest> {
        Self::parse_test(data)
    }

    fn test_a() -> anyhow::Result<()> {
        assert_eq!(
            Self::a_test(Self::final_parse_test(Self::SAMPLE_DATA)?)?,
            Self::SAMPLE_ANSWER_A
        );
        println!("a: {}", Self::part1(Self::final_parse(Self::DATA)?)?);
        Ok(())
    }

    fn test_b() -> anyhow::Result<()> {
        assert_eq!(
            Self::b_test(Self::final_parse_test(Self::SAMPLE_DATA_B)?)?,
            Self::SAMPLE_ANSWER_B
        );
        println!("b: {}", Self::part2(Self::final_parse(Self::DATA)?)?);
        Ok(())
    }

    fn main() -> anyhow::Result<()> {
        let parsed = Self::final_parse(Self::DATA)?;
        let arg = std::env::args().nth(1);
        match arg.as_deref() {
            Some("a") => {
                let now = Instant::now();
                println!("a: {} ({:?})", Self::part1(parsed)?, now.elapsed());
            }
            Some("b") => {
                let now = Instant::now();
                println!("b: {} ({:?})", Self::part2(parsed)?, now.elapsed());
            }
            _ => {
                let now_a = Instant::now();
                println!(
                    "a: {} ({:?})",
                    Self::part1(parsed.clone())?,
                    now_a.elapsed()
                );
                let now_b = Instant::now();
                println!("b: {} ({:?})", Self::part2(parsed)?, now_b.elapsed());
            }
        }
        Ok(())
    }
}
