use boilerplate::run_day;

mod boilerplate;
mod input_util;
mod solutions;

fn main() {
    println!("My advent of code 2024 adventure");

    let first = 1;
    let last = 2;
    for day_number in first..=last {
        println!("======== Day {} =========", day_number);
        run_day(day_number);
    }
}
