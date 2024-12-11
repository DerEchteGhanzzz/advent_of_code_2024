use regex::Regex;

pub fn solve_a(input: &Vec<String>) -> i64 {
    sum_mults(parse_do_mults(&input[0])) as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    apply_do_donts(parse_do_mults(&input[0])) as i64
}

fn sum_mults(operations: Vec<Operation>) -> i32 {
    let mut total = 0;
    for op in operations {
        total += op.apply();
    }
    total
}

fn apply_do_donts(operations: Vec<Operation>) -> i32 {
    let mut dont = false;
    let mut total = 0;
    for op in operations {
        if !dont {
            total += op.apply();
        }
        match op {
            Operation::Do => dont = false,
            Operation::Dont => dont = true,
            _ => (),
        }
    }
    total
}

fn parse_do_mults(line: &str) -> Vec<Operation> {
    let mut operations = Vec::new(); 
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    
    for c in re.find_iter(line) {
        let op = Operation::new(c.as_str());
        operations.push(op);
    }
    operations
}

#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Operation {

    pub fn new(op: &str) -> Self {
        match op {
            "do()" => Self::Do,
            "don't()" => Self::Dont,
            _ => Self::new_mul(op),
        }
    }

    pub fn new_mul(op: &str) -> Self {
        let numbers = op.split(",").map(
            |s| s.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap()
        ).collect::<Vec<_>>();
        return Self::Mul(
            numbers[0],
            numbers[1]
        );
    }

    pub fn apply(&self) -> i32 {
        match self {
            Self::Mul(x, y) => x * y,
            _ => 0,
        }
    }
}