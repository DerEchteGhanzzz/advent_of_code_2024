use std::collections::{HashMap, VecDeque};

use super::point::Point;

pub fn solve_a(input: &Vec<String>) -> i64 {
    let map = map_to_hashmap(input);
    let starts = map.iter().filter(|(_, c)| **c == 0);
    starts.flat_map(|(start, _)| breadth_first_trail(start, 9, &map, false)).count() as i64
}


pub fn solve_b(input: &Vec<String>) -> i64 {
    let map = map_to_hashmap(input);
    let starts = map.iter().filter(|(_, c)| **c == 0);
    starts.flat_map(|(start, _)| breadth_first_trail(start, 9, &map, true)).count() as i64
}


fn map_to_hashmap(input: &Vec<String>) -> HashMap<Point, i64> {

    input.iter().enumerate().flat_map(|(y, s)| s.chars().enumerate().map(move |(x, c)| (Point::new(x as i64, y as i64), c.to_digit(10).unwrap() as i64))).collect::<HashMap<_, _>>()
}


fn breadth_first_trail(start: &Point, end: i64, map: &HashMap<Point, i64>, is_b: bool) -> Vec<Point> {
    let mut queue = VecDeque::from([start.clone()]);
    let mut trail_ends = Vec::new();
    let mut prev = HashMap::from([(start.clone(), (None, 0))]);

    while !queue.is_empty() {

        let current = queue.pop_front().unwrap();

        for neigh in current.get_neighs() {
            if !map.contains_key(&neigh) || (prev.contains_key(&neigh) && !is_b)
            {
                continue;
            }
            if *map.get(&neigh).unwrap() != *map.get(&current).unwrap() + 1 {
                continue;
            }
            if *map.get(&neigh).unwrap() == end {
                trail_ends.push(neigh.clone());
            }
            queue.push_back(neigh.clone());
            prev.insert(neigh.clone(), (Some(current.clone()), prev.get(&current).unwrap().1));
        }

    }
    trail_ends
}