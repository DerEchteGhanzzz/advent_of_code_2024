use parser::get_input;
mod days;

mod parser;

fn main() {
    println!("{}", days::day_x::solve_a(get_input("X")));
    println!("{}", days::day_x::solve_b(get_input("X")));
}
