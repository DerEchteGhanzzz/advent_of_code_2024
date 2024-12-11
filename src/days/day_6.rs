use std::{collections::HashSet, ops::{Add, Sub}};

pub fn solve_a(input: &Vec<String>) -> i64 {
    let map = get_map(input);
    let guard_position = get_guard_position(&map);
    walk_the_guard2(guard_position.clone(), Direction::Up, HashSet::new(), &map).iter().map(|(p, _)| p).collect::<HashSet<_>>().len() as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let map = get_map(input);
    let guard_position = get_guard_position(&map);
    loop_the_guard2(guard_position, Direction::Up, HashSet::new(), &map) as i64
}

fn loop_the_guard2(mut position: Point, mut dir: Direction, mut visited: HashSet<(Point, Direction)>, map: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut has_had = HashSet::new();
    let (x_min, x_max, y_min, y_max) = (0, map[0].len() as i32 - 1, 0, map.len() as i32 - 1);
    while !(position.clone() + dir.to_point()).out_of_bounds(x_min, x_max, y_min, y_max) {
        
        let (new_position, new_dir) = do_one_step(position.clone(), dir.clone(), &mut visited, map);

        if has_had.insert(new_position.clone()) && loop_the_guard(position, dir.turn_right(), &mut HashSet::new(), map, new_position.clone()) {
            count += 1;
        }
        
        position = new_position;
        dir = new_dir;
    }
    count
}

fn walk_the_guard2(mut position: Point, mut dir: Direction, mut visited: HashSet<(Point, Direction)>, map: &Vec<Vec<char>>) -> HashSet<(Point, Direction)> {
    let (x_min, x_max, y_min, y_max) = (0, map[0].len() as i32 - 1, 0, map.len() as i32 - 1);
    while !(position.clone() + dir.to_point()).out_of_bounds(x_min, x_max, y_min, y_max) {
        (position, dir) = do_one_step(position, dir, &mut visited, map);
    }
    visited.insert((position, dir));
    visited
}

fn do_one_step(position: Point, dir: Direction, visited: &mut HashSet<(Point, Direction)>, map: &Vec<Vec<char>>) -> (Point, Direction) {
    visited.insert((position.clone(), dir.clone()));
    let next_pos = position.clone() + dir.to_point();
    
    let new_dir = if map[next_pos.y() as usize][next_pos.x() as usize] == '#' { dir.turn_right() } else { dir };
    let new_pos = position + new_dir.to_point();

    (new_pos, new_dir)
}

fn loop_the_guard(guard: Point, dir: Direction, visited: &mut HashSet<(Point, Direction)>, map: &Vec<Vec<char>>, obstruction: Point) -> bool {
    let (x_min, x_max, y_min, y_max) = (0, map[0].len() as i32 - 1, 0, map.len() as i32 - 1);
    if guard.out_of_bounds(x_min, x_max, y_min, y_max) {
        return false
    }
    
    let mut position = guard;
    loop {
        let new_pos = position.clone() + dir.to_point();
        if new_pos.out_of_bounds(x_min, x_max, y_min, y_max) {
            return false
        }
        if map[new_pos.y() as usize][new_pos.x() as usize] == '#' || new_pos == obstruction {
            break;
        }
        position = new_pos;
        if !visited.insert((position.clone(), dir.clone())) {
            return true
        }
    }
    loop_the_guard(position, dir.turn_right(), visited, map, obstruction)
}

fn get_guard_position(map: &Vec<Vec<char>>) -> Point {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                return Point::new(x as i32, y as i32);
            }
        }
    }
    Point::new(-1, -1)
}

fn get_map(input: &Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {

    pub fn turn_right(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn to_point(&self) -> Point {
        match self {
            Self::Up => Point::new(0, -1),
            Self::Right => Point::new(1, 0),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
        }
    }
}