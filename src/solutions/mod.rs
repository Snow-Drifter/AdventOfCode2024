mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_09;
mod day_10;

use day_01::*;
use day_02::*;
use day_03::*;
use day_04::*;
use day_09::*;
use day_10::*;

pub fn run_day(day_number: usize) -> (i64, i64) {
    let input_directory = get_file_path(day_number);
    match day_number {
        1 => run_solution::<Day01>(&input_directory),
        2 => run_solution::<Day02>(&input_directory),
        3 => run_solution::<Day03>(&input_directory),
        4 => run_solution::<Day04>(&input_directory),
        9 => run_solution::<Day09>(&input_directory),
        10 => run_solution::<Day10>(&input_directory),
        _ => unimplemented!("Day not implemented for {}", day_number),
    }
}

pub fn run_solution<S: Solution>(file_path: &str) -> (i64, i64) {
    (
        S::part_one(file_path),
        S::part_two(file_path),
    )
}

pub fn get_file_path(day: usize) -> String {
    match day {
        1..=9 => format!("src/res/day_0{}.txt", day),
        10..=25 => format!("src/res/day_{}.txt", day),
        _ => panic!("Only designed for 25 days"),
    }
}

pub trait Solution {
    fn part_one(file_path: &str) -> i64;

    fn part_two(file_path: &str) -> i64;
}
