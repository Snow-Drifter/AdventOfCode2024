use crate::{input_util, boilerplate::Solution};

use regex::Regex;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input_dir: &str) -> i64 {
        let (mut old, mut new): (Vec<_>, Vec<_>) = input_util::read_file_buffered(input_dir)
            .map(|line_read| {
                let line = line_read.expect("The input file is parsable");
                let pattern = Regex::new(r"(\d+)\s+(\d+)").unwrap();
                let captures = pattern.captures(&line).unwrap();
                let left = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let right = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                (left, right)
            })
            .unzip();
        old.sort();
        new.sort();
        old.iter().zip(new.iter()).map(|(a, b)| (a - b).abs()).sum()
    }

    fn part_two(&self, input_dir: &str) -> i64 {
        0
    }
}
