// 2021 day1 exercise
pub fn solve_a(input: Vec<String>) -> i32 {
    let depths = to_int_vec(&input);
	depths.windows(2).fold(0 ,|acc, pair| if pair[1] > pair[0] { acc + 1 } else { acc } )
}

pub fn solve_b(input: Vec<String>) -> i32 {
    let depths = to_int_vec(&input);
	depths.windows(3).fold(0, |acc, pair| if (pair[1] + pair[2]) > (pair[0] + pair[1]) { acc + 1 } else { acc })
}

pub fn to_int_vec(input: &Vec<String>) -> Vec<i32> {
    input.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()
}