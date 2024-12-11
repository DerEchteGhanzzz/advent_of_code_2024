use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::days::point::Point;

pub fn solve_a(input: &Vec<String>) -> i64 {
    let antenna_map = get_antenna_dict(input);
    let (x_min, x_max, y_min, y_max) = (0, input[0].len() as i64 - 1, 0, input.len() as i64 - 1);
    antenna_map.into_values().into_iter().flat_map(|v| calc_resonance_points(v)).filter(|p| !p.out_of_bounds(x_min, x_max, y_min, y_max)).collect::<HashSet<_>>().len() as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let antenna_map = get_antenna_dict(input);
    let (x_min, x_max, y_min, y_max) = (0, input[0].len() as i64 - 1, 0, input.len() as i64 - 1);
    antenna_map.into_values().into_iter().flat_map(|v| calc_many_resonance_points(v)).filter(|p| !p.out_of_bounds(x_min, x_max, y_min, y_max)).collect::<HashSet<_>>().len() as i64
}

fn calc_resonance_points(v: Vec<Point>) -> Vec<Point> {
    v.iter().combinations(2).flat_map(|combs| vec![combs[0].clone() + combs[0].clone() - combs[1].clone(), combs[1].clone() + combs[1].clone() - combs[0].clone()]).collect()
}


fn calc_many_resonance_points(v: Vec<Point>) -> Vec<Point> {
    v.iter().combinations(2).flat_map(|combs| (0..50).flat_map(|scale| vec![combs[0].clone() + (combs[0].clone() - combs[1].clone()).scale(scale), combs[1].clone() + (combs[1].clone() - combs[0].clone()).scale(scale)]).collect::<Vec<_>>()).collect()
}


fn get_antenna_dict(input: &Vec<String>) -> HashMap<char, Vec<Point>> {
    let map = input.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut antenna_map = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let antenna = map[y][x];
            if !antenna.is_alphanumeric() {
                continue;
            }
            if !antenna_map.contains_key(&antenna) {
                antenna_map.insert(antenna,  vec![Point::new(x as i64, y as i64)]);
            } else {
                antenna_map.get_mut(&antenna).unwrap().push(Point::new(x as i64, y as i64));
            }
        }
    }
    antenna_map
}