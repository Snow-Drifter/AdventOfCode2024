use crate::{boilerplate::Solution, input_util};

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input_dir: &str) -> i64 {
        let reports = Self::parse_lines(input_dir);
        reports
            .iter()
            .filter(|report| Self::increasing(report) || Self::decreasing(report))
            .count() as i64
    }

    fn part_two(&self, input_dir: &str) -> i64 {
        let reports = Self::parse_lines(input_dir);
        reports
            .iter()
            .filter(|report| Self::lenient_increasing(report) || Self::lenient_decreasing(report))
            .count() as i64
    }
}

impl Day02 {
    fn increased(pair: &[i64]) -> bool {
        let left = pair[0];
        let right = pair[1];
        right > left && left + 4 > right
    }
    fn decreased(pair: &[i64]) -> bool {
        let left = pair[0];
        let right = pair[1];
        right < left && right + 4 > left
    }
    fn increasing(report: &[i64]) -> bool {
        report.windows(2).all(Self::increased)
    }
    fn decreasing(report: &[i64]) -> bool {
        report.windows(2).all(Self::decreased)
    }
    fn lenient_increasing(report: &[i64]) -> bool {
        let pairs = report.windows(2);
        if let Some(broken) = pairs
            .enumerate()
            .find(|(_, pair)| !Self::increased(pair))
            .map(|(index, _)| index)
        {
            let lists = vec![
                [&report[0..broken], &report[broken + 1..]].concat(),
                [&report[0..broken + 1], &report[broken + 2..]].concat(),
            ];
            lists.iter().any(|list| Self::increasing(list))
        } else {
            true
        }
    }
    fn lenient_decreasing(report: &[i64]) -> bool {
        let pairs = report.windows(2);
        if let Some(broken) = pairs
            .enumerate()
            .find(|(_, pair)| !Self::decreased(pair))
            .map(|(index, _)| index)
        {
            let lists = vec![
                [&report[0..broken], &report[broken + 1..]].concat(),
                [&report[0..broken + 1], &report[broken + 2..]].concat(),
            ];
            lists.iter().any(|list| Self::decreasing(list))
        } else {
            true
        }
    }
    fn parse_lines(input_dir: &str) -> Vec<Vec<i64>> {
        input_util::read_file_buffered(input_dir)
            .map(|line_read| {
                line_read
                    .expect("The input file is parsable")
                    .split_whitespace()
                    .map(|number_str| number_str.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}
