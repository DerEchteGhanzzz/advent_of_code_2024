use std::collections::HashMap;

pub fn solve_a(input: &Vec<String>) -> i64 {
    start_blinking(input[0].split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>(), 25)
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    start_blinking(input[0].split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>(), 75)
}

fn start_blinking(numbers: Vec<i64>, amount: i64) -> i64 {
    let mut cache = HashMap::new();
    numbers.into_iter().map(|n| blink(n, amount, &mut cache)).sum()
}

fn blink(number: i64, depth: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if depth == 0 {
        return 1;
    }
    if cache.contains_key(&(number, depth)) {
        return *cache.get(&(number, depth)).unwrap();
    }
    if number == 0 {
        let ans = blink(1, depth - 1, cache);
        cache.insert((number, depth), ans);
        return ans;
    }
    if number.to_string().len() % 2 == 0 {
        let num_str = number.to_string();
        let left = num_str[0..num_str.len()/2].parse::<i64>().unwrap();
        let right = num_str[num_str.len()/2..num_str.len()].parse::<i64>().unwrap();
        
        let ans = blink(left, depth - 1, cache) + blink(right, depth - 1, cache);
        cache.insert((number, depth), ans);
        return ans;
    }
    let ans = blink(number * 2024, depth - 1, cache);
    cache.insert((number, depth), ans);
    return ans
}