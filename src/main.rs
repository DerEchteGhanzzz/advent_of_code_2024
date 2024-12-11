use std::time::Instant;
use parser::get_input;
mod days;

mod parser;

fn run_day(day: &str, solve_a: fn(&Vec<String>) -> i64, solve_b: fn(&Vec<String>) -> i64) {
    println!("Day {}:", day);
    let mut start = Instant::now();
    let input = get_input(day);
    println!("parsing in: {:?}", start.elapsed());
    start = Instant::now();
    println!("Solution A: {} in: {:?}", solve_a(&input), start.elapsed());
    start = Instant::now();
    println!("Solution B: {} in: {:?}", solve_b(&input), start.elapsed());
    println!("");
}

fn main() {
    run_day("1", days::day_1::solve_a, days::day_1::solve_b);
    run_day("2", days::day_2::solve_a, days::day_2::solve_b);
    run_day("3", days::day_3::solve_a, days::day_3::solve_b);
    run_day("4", days::day_4::solve_a, days::day_4::solve_b);
    run_day("5", days::day_5::solve_a, days::day_5::solve_b);
    // run_day("6", days::day_6::solve_a, days::day_6::solve_b);
    run_day("7", days::day_7::solve_a, days::day_7::solve_b);
    run_day("8", days::day_8::solve_a, days::day_8::solve_b);
    // run_day("9", days::day_9::solve_a, days::day_9::solve_b);
    run_day("10", days::day_10::solve_a, days::day_10::solve_b);
    run_day("11", days::day_11::solve_a, days::day_11::solve_b);
}
