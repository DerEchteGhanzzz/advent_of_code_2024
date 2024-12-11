use core::num;
use std::{collections::{HashMap, HashSet, VecDeque}, ops::{Add, Sub}};

pub fn solve_a(input: &Vec<String>) -> i64 {
    let mut ans = 0;
    for (target, numbers) in parse_input(input) {
        if check_sum(target, numbers.clone()) {
            ans += target;
        }
    }
    ans
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let mut ans = 0;
    for (target, numbers) in parse_input(input) {
        if check_sum(target, numbers.clone()) {
            ans += target;
        }
        else if check_sum2(target, numbers) {
            ans += target;
        }
    }
    ans
}

fn check_sum(target: i64, numbers: Vec<i64>) -> bool {

    let mut operations = HashMap::from([]);
    let mut prev = HashMap::from([(numbers.len(), HashSet::from([target]))]);
    for (idx, number) in numbers.iter().enumerate().rev() {
        let mut new_targets = HashSet::new();
        // println!("{:?}", prev.get(&(idx + 1)).unwrap());
        // println!("{:?} {:?}", idx, number);
        for sub_target in prev.get(&(idx + 1)).unwrap() {
            if sub_target - number >= 0 {
                new_targets.insert(sub_target-number);
                operations.insert(sub_target - number, Op::Add);
            }
            if sub_target % number == 0 {
                new_targets.insert(sub_target / number);
                operations.insert(sub_target / number, Op::Mul);
            }
        }
        // println!("new: {:?}", new_targets);
        prev.insert(idx, new_targets);
    }
    // println!("prev: {:?}", prev.get(&0));
    prev.get(&0).unwrap().contains(&0)
}

fn check_sum2(target: i64, numbers: Vec<i64>) -> bool {

    let mut operations = HashMap::from([]);
    let mut prev = HashMap::from([(numbers.len(), HashSet::from([target]))]);
    for (idx, number) in numbers.iter().enumerate().rev() {
        let mut new_targets = HashSet::new();
        for sub_target in prev.get(&(idx + 1)).unwrap() {
            if sub_target - number >= 0 {
                new_targets.insert(sub_target-number);
                operations.insert(sub_target - number, Op::Add);
            }
            if sub_target % number == 0 {
                new_targets.insert(sub_target / number);
                operations.insert(sub_target / number, Op::Mul);
            }
            let target_str = sub_target.to_string();
            if target_str.ends_with(&numbers[idx].to_string()) && numbers[idx].to_string().len() < target_str.len() {
                let slice = &target_str[0..target_str.len() - numbers[idx].to_string().len()];
                // println!("slice: {}", slice);
                new_targets.insert(slice.parse::<i64>().unwrap());
            }
        }
        prev.insert(idx, new_targets);
    }
    prev.get(&0).unwrap().contains(&0)
}

fn parse_input(input: &Vec<String>) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.iter().map(|s| {let split = s.split(": ").collect::<Vec<_>>(); (split[0].parse::<i64>().unwrap(), split[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()) })
}

enum Op {
    Add,
    Mul,
}