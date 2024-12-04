use crate::solutions::Solution;
use regex::{Captures, Regex};
use crate::input_util;

pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input_dir: &str) -> i64 {
        let functions = Self::parse_lines(input_dir);
        functions
            .iter()
            .map(|token| {
                if let Token::Mul(x, y) = token {
                    x * y
                } else {
                    0
                }
            })
            .sum()
    }

    fn part_two(&self, input_dir: &str) -> i64 {
        Self::parse_lines(input_dir)
            .iter()
            .fold((0, true), |(sum, enabled), token| match (token, enabled) {
                (Token::Mul(x, y), true) => (sum + x * y, true),
                (Token::Do, false) => (sum, true),
                (Token::Dont, true) => (sum, false),
                (_, _) => (sum, enabled),
            })
            .0
    }
}

enum Token {
    Do,
    Dont,
    Mul(i64, i64),
}

impl From<Captures<'_>> for Token {
    fn from(value: Captures) -> Self {
        if let Some(switch) = value.name("switch") {
            match switch.as_str() {
                "do()" => Token::Do,
                "don't()" => Token::Dont,
                _ => unreachable!(),
            }
        } else {
            Token::Mul(
                value["left"].parse().unwrap(),
                value["right"].parse().unwrap(),
            )
        }
    }
}

impl Day03 {
    fn parse_lines(input_dir: &str) -> Vec<Token> {
        let mul_pattern =
            Regex::new(r"(?<switch>do\(\)|don't\(\))|mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
        input_util::read_file_buffered(input_dir)
            .flat_map(|line_read| {
                let line = line_read.unwrap();
                mul_pattern
                    .captures_iter(&line)
                    .map(|capture| Token::from(capture))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
