// 2021 day1 exercise
pub fn solve_a(input: Vec<String>) -> u64 {
    input.iter().map(|chunk| match parse_parens(&chunk) {
        Err(ch) => to_score(ch),
        _ => 0,
    }).sum()
}

pub fn solve_b(input: Vec<String>) -> u64 {
    let mut scores = input.iter().filter_map(|chunk| {
        match parse_parens(chunk) {
            Err(_) => None,
            Ok(result) => {
                Some(result.iter().rev().fold(0, |total, ch| total * 5 + to_score_b(*ch)))
            }
        }
    }).collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}

fn parse_parens(parens_str: &str) -> Result<Vec<char>, char> {
    
    parens_str.chars().fold(Ok(Vec::new()) ,|acc, ch| {
        match acc {
            Err(_) => acc,
            Ok(mut stack) =>
                match ch {
                    '(' | '[' | '{' | '<' => {stack.push(reverse(ch)); Ok(stack)},
                    _   => match stack.pop() {
                        None => Err('x'),
                        Some(p) => { if ch == p { Ok(stack) } else { Err(ch) } }
                    }
                }
            }
        }
    )
}

fn reverse(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => 'x',
    }
}

fn to_score(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn to_score_b(ch: char) -> u64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}