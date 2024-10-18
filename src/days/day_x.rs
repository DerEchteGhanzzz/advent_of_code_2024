// last year's day1 for exercise
pub fn solve_a(input: Vec<String>) -> String {
	let values = input.iter().map(|l| int_to_calibration_value(l)).collect::<Vec<u64>>();
	return format!("{}", values.iter().sum::<u64>());
}

pub fn solve_b(input: Vec<String>) -> String {
	let values = input.iter().map(|l| calibration_value(l)).collect::<Vec<u64>>();
	return format!("{}", values.iter().sum::<u64>());
}

fn int_to_calibration_value(line: &str) -> u64 {
	let chars: Vec<_> = line.chars().collect();
	let idx_1 = chars.iter().position(|c| c.is_numeric()).unwrap();
	let idx_2 = chars.len() - 1 - chars.iter().rev().position(|c| c.is_numeric()).unwrap();
	return format!("{}{}", chars[idx_1], chars[idx_2]).parse().unwrap();
}

fn calibration_value(line: &str) -> u64 {
	let result = 
		line
		.replace("one", "one1one")
		.replace("two", "two2two")
		.replace("three", "three3three")
		.replace("four", "four4four")
		.replace("five", "five5five")
		.replace("six", "six6six")
		.replace("seven", "seven7seven")
		.replace("eight", "eight8eight")
		.replace("nine", "nine9nine")
		.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

	return format!("{}{}", result[0], result[result.len()-1]).parse().unwrap();
}