pub fn solve_a(input: &Vec<String>) -> i64 {
    let (mut left_list, mut right_list) = split_lists(input);
    left_list.sort();
    right_list.sort();
    left_list.iter().zip(right_list.iter()).fold(0, |a, (l, r)| a + (l - r).abs()) as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let (mut left_list, right_list) = split_lists(input);
    left_list.sort();
    left_list.into_iter().fold(0, |a, l| a + l * freq(l, &right_list).abs()) as i64
}

fn freq(x: i32, r: &Vec<i32>) -> i32 {
    r.iter().filter(|y| **y == x).collect::<Vec<_>>().len() as i32
}

fn split_lists(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list = Vec::new();
    for entry in input {
        let split = entry.split(' ').collect::<Vec<_>>();
        let left = split.first().unwrap().to_string().parse::<i32>().unwrap();
        let right = split.last().unwrap().to_string().parse::<i32>().unwrap();
        left_list.push(left);
        right_list.push(right);
    }
    return (left_list, right_list)
}