use std::time::Instant;
use cached::proc_macro::cached;
use crate::input_util;
use crate::solutions::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_one(dir: &str) -> i64 {
        let mut cells = Self::parse(dir);
        let start = Instant::now();
        for _blink in 0..25 {
            cells = cells.iter().flat_map(|cell| cell.next()).collect();
            // println!("Part 1 Blink {} Finished", _blink + 1);
        }
        println!("Part 1 Time: {}ms", start.elapsed().as_millis());
        cells.len() as i64
    }

    fn part_two(dir: &str) -> i64 {
        let mut cells = Self::parse(dir);
        for _blink in 0..75 {
            let start = Instant::now();
            cells = cells.iter().flat_map(|cell| cell.next()).collect();
            println!("Part 2 Blink {} Finished in {}ms", _blink + 1, start.elapsed().as_millis());
        }
        cells.len() as i64
    }
}

impl Day11 {
    fn parse(dir: &str) -> Vec<Cell> {
        input_util::read_file_buffered(dir)
            .map(|maybe_line| maybe_line.unwrap())
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|word| word.parse::<usize>().expect("number"))
                    .collect::<Vec<_>>()
            })
            .map(|num| num.into())
            .collect()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Cell(usize, usize);

impl From<usize> for Cell {
    fn from(num: usize) -> Self {
        Self(num, 0)
    }
}

impl Cell {
    fn next(&self) -> Vec<Cell> {
        next_num(self.0).into_iter().map(Cell::from).collect()
    }
}

#[cached]
pub fn next_num(num: usize) -> Vec<usize> {
    if num == 0 {
        vec![1]
    } else {
        let digits = num.to_string();
        let length = digits.len();
        if length % 2 != 0 {
            vec![num * 2024]
        } else {
            let (left, right) = digits.split_at(length / 2);
            vec![left, right]
                .iter()
                .map(|digits| digits.parse::<usize>().expect("number").into())
                .collect()
        }
    }
}

#[test]
fn blink_six_times() {
    let mut initial: Vec<Cell> = vec![125.into(), 17.into()];
    for _blink in 0..6 {
        initial = initial.iter().flat_map(|cell| cell.next()).collect();
        println!("After blink: {}. ({:?})", _blink, initial);
    }
    assert_eq!(
        initial,
        vec![
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2
        ]
        .iter()
        .map(|&num| num.into())
        .collect::<Vec<_>>()
    );
}
