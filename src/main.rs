use std::time::Instant;
use parser::get_input;
mod days;

mod parser;

fn main() {
    let start = Instant::now();

    println!("Solution A:\n{}", days::day_x::solve_a(get_input("X")));
    println!("Solution B:\n{}", days::day_x::solve_b(get_input("X")));
    
    println!("Time elapsed: {:?}", start.elapsed());
}
