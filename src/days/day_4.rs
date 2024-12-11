use std::ops::{Add, Sub};

pub fn solve_a(input: &Vec<String>) -> i64 {
    find_occurrances(&input.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>(), "XMAS") as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    find_x_masses(&input.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>(), "MAS") as i64
}

fn find_occurrances(map: &Vec<Vec<char>>, pattern: &str) -> i32 {
    let mut count = 0;
    let (x_min, x_max, y_min, y_max) = (0, map[0].len() as i32 - 1, 0, map.len() as i32 - 1);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let current = Point::new(x as i32, y as i32);
            for j in -1..=1 {
                for i in -1..=1 {

                    if i == j && j == 0 { continue; }
                    let direction = Point::P2{x: i, y: j};
                    let mut stack = pattern.chars().rev().collect::<Vec<_>>();
                    let mut target = current.clone();
                    while
                        !target.out_of_bounds(x_min, x_max, y_min, y_max) && 
                        !stack.is_empty() && 
                        *stack.last().unwrap() == map[target.y() as usize][target.x() as usize]
                    {
                        target = target + direction.clone();
                        stack.pop();
                    }
                    
                    if stack.is_empty() {
                        count += 1;
                    }
                }
            }

        }
    }
    count
}

fn find_x_masses(map: &Vec<Vec<char>>, pattern: &str) -> i32 {
    let mut count = 0;
    let (x_min, x_max, y_min, y_max) = (0, map[0].len() as i32 - 1, 0, map.len() as i32 - 1);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let current = Point::new(x as i32, y as i32);
            let mut local_count = 0;
            for direction in vec![Point::P2{x: 1, y: 1}, Point::P2{x: 1, y: -1}, Point::P2{x: -1, y: -1}, Point::P2{x: -1, y: 1}] {

                let mut stack = pattern.chars().rev().collect::<Vec<_>>();
                let mut target = current.clone() - direction.clone();
                while
                    !target.out_of_bounds(x_min, x_max, y_min, y_max) && 
                    !stack.is_empty() && 
                    *stack.last().unwrap() == map[target.y() as usize][target.x() as usize]
                {
                    target = target + direction.clone();
                    stack.pop();
                }
                
                if stack.is_empty() {
                    local_count += 1;
                }
            }
            if local_count == 2 {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, Clone)]
enum Point {
    P2{x: i32, y: i32},
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self::P2{x, y}
    } 

    pub fn out_of_bounds(&self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> bool {
        let Point::P2{x, y} = self;
        *x < x_min || x_max < *x || *y < y_min || y_max < *y
    }

    pub fn x(&self) -> i32 {
        let Self::P2{x, y: _} = self;
        *x
    }

    pub fn y(&self) -> i32 {
        let Self::P2{x: _, y} = self;
        *y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Point::P2{x: x1, y: y1} = self;
        let Point::P2{x: x2, y: y2} = rhs;
        Self::new(x1+x2, y1+y2)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Point::P2{x: x1, y: y1} = self;
        let Point::P2{x: x2, y: y2} = rhs;
        Self::new(x1-x2, y1-y2)
    }
}