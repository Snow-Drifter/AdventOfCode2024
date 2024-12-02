use crate::{input_util, solutions};

pub fn run_day(day_number: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => run(solutions::day_01::Day01 {}, &file_path),
        _ => todo!(),
    }
}

pub fn run<S: Solution>(day_solution: S, file_path: &str) {
    let answer = day_solution.part_one(file_path);

    println!("P1: {}", answer);

    let answer = day_solution.part_two(file_path);

    println!("P2: {}", answer);
}

pub trait Solution {
    fn part_one(&self, file_path: &str) -> i64;

    fn part_two(&self, file_path: &str) -> i64;
}
