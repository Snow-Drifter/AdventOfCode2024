mod day_01;
mod day_02;
mod day_03;

pub fn run_day(day_number: usize) -> (i64, i64) {
    let input_directory = get_file_path(day_number);
    match day_number {
        1 => run_solution(day_01::Day01, &input_directory),
        2 => run_solution(day_02::Day02, &input_directory),
        3 => run_solution(day_03::Day03, &input_directory),
        _ => unimplemented!("Day not implemented for {}", day_number),
    }
}

pub fn run_solution<S: Solution>(day_solution: S, file_path: &str) -> (i64, i64) {
    (
        day_solution.part_one(file_path),
        day_solution.part_two(file_path),
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
    fn part_one(&self, file_path: &str) -> i64;

    fn part_two(&self, file_path: &str) -> i64;
}
