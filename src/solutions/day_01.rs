use crate::input_util;
use std::collections::HashMap;
use std::ops::AddAssign;
use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(input_dir: &str) -> i64 {
        let (mut left_list, mut right_list) = Self::parse_lines(input_dir);
        left_list.sort();
        right_list.sort();
        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn part_two(input_dir: &str) -> i64 {
        let (left_list, right_list) = Self::parse_lines(input_dir);
        let mut frequency: HashMap<i64, i64> = HashMap::default();
        right_list
            .iter()
            .for_each(|&right| frequency.entry(right).or_default().add_assign(1));
        left_list
            .iter()
            .map(|left| frequency.get(left).unwrap_or(&0) * left)
            .sum()
    }
}

impl Day01 {
    fn parse_lines(input_dir: &str) -> (Vec<i64>, Vec<i64>) {
        input_util::read_file_buffered(input_dir)
            .map(|line_read| {
                let pair: Vec<i64> = line_read
                    .expect("The input file is parsable")
                    .split_whitespace()
                    .map(|number_str| number_str.parse().unwrap())
                    .collect();
                (pair[0], pair[1])
            })
            .unzip()
    }
}
