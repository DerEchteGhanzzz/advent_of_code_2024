use std::convert::identity;

use itertools::Itertools;

pub fn solve_a(input:& Vec<String>) -> i64 {
    input.iter().map(|line| 
        Report::new(line_to_ints(line)).is_safe()).filter(|b| *b).count() as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    input.iter().map(|line| 
        Report::new(line_to_ints(line)).tolerated()).filter(|b| *b).count() as i64
}

struct Report {
    levels: Vec<i32>,
}

impl Report {

    pub fn new(levels: Vec<i32>) -> Self {
        Report { levels }
    }

    pub fn is_safe(&self) -> bool {
        let sign = (self.levels[1] - self.levels[0]).signum();
        self.levels.iter().tuple_windows().all(|(p, c)| 1 <= (c-p).abs() && (c-p).abs() <= 3 && (c-p).signum() == sign)
    }

    pub fn tolerated(&self) -> bool {
        self.levels.clone().into_iter().combinations(self.levels.len()-1).map(|lvl| Report::new(lvl).is_safe()).any(identity)
    }
}

fn line_to_ints(line: &String) -> Vec<i32> {
    line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()
}