use std::env;

mod input_util;
mod solutions;

fn main() {
    let days_implemented = [10];

    println!("My advent of code 2024 adventure");
    let args: Vec<String> = env::args().collect();
    if let Some(day_number) = args.get(1).map(|arg| arg.parse::<usize>().ok()).flatten() {
        // run specified
        if days_implemented.contains(&day_number) {
            println!("======== Day {} =========", day_number);
            let (part1, part2) = solutions::run_day(day_number);
            println!("P1: {}", part1);
            println!("P2: {}", part2);
        } else {
            panic!("Day {} not implemented", day_number);
        }
    } else {
        // run all
        for day_number in days_implemented {
            println!("======== Day {} =========", day_number);
            let (part1, part2) = solutions::run_day(day_number);
            println!("P1: {}", part1);
            println!("P2: {}", part2);
        }
    }
}
