use std::collections::HashMap;

pub fn solve_a(input: &Vec<String>) -> i64 {
    let mut files = to_byte_string(&input[0]);
    let mut first_empty_idx = 1;
    let mut last_full_idx = files.len() - 1;

    while first_empty_idx < last_full_idx {
        if files[last_full_idx].0.is_none() {
            last_full_idx -= 1;
            continue;
        }
        if files[first_empty_idx].0.is_some() {
            first_empty_idx += 1;
            continue;
        }
        (first_empty_idx, last_full_idx) = move_one(&mut files, first_empty_idx, last_full_idx);
    }
    calc_checksum(&files.into_iter().take_while(|(id, _)| id.is_some()).collect::<Vec<_>>())
}


pub fn solve_b(input: &Vec<String>) -> i64 {
    
    let mut files = to_byte_string(&input[0]);
    let last_file_id = if files[files.len() - 1].0.is_none() { files[files.len()-2].0 } else { files[files.len() - 1].0 }.unwrap();
    
    // let idx_map = get_idx_map(&files);
    let mut insertion_count = 0;
    for file_id in (0..=last_file_id).rev() {
        let file_idx = get_idx_of_id(&files, file_id);
        // let file_idx = *idx_map.get(&file_id).unwrap();
        // let file_idx = file_id*2;
        if try_move(&mut files, file_idx) {
            insertion_count += 1;
        }
    }
    calc_checksum(&files)
}

fn get_idx_map(files: &Vec<(Option<usize>, i64)>) -> HashMap<usize, usize> {
    let mut idx_map = HashMap::new();

    for (idx, (id, _)) in files.iter().enumerate() {
        if id.is_some() {
            idx_map.insert(id.unwrap(), idx);
        }
    }
    return idx_map;
}

fn get_idx_of_id(files: &Vec<(Option<usize>, i64)>, file_id: usize) -> usize {
    for (idx, (id, _)) in files.iter().enumerate() {
        if id.is_some() && id.unwrap() == file_id {
            return idx;
        }
    }
    return files.len()
}

fn to_byte_string(input: &String) -> Vec<(Option<usize>, i64)> {
    input.chars().enumerate().map(|(id, size)| if id % 2 == 1 { (None, size.to_digit(10).unwrap() as  i64) } else { (Some(id / 2), size.to_digit(10).unwrap() as  i64) }).collect::<Vec<_>>()
}

fn try_move(files: &mut Vec<(Option<usize>, i64)>, file_idx: usize) -> bool {
    let full = files[file_idx];
    for (first_empty_idx, (id, size)) in files.iter().enumerate() {

        if first_empty_idx > file_idx { return false; }

        if id.is_none() && size >= &full.1 {

            let mut empty = files[first_empty_idx];
            
            if empty.1 == full.1 {
                files[first_empty_idx] = full;
                files[file_idx] = (None, full.1);
            } else if empty.1 > full.1 {
                files[file_idx] = (None, full.1);
                files.insert(first_empty_idx, full);
                empty = (None, empty.1 - full.1);
                files[first_empty_idx + 1] = empty;
                return true;
            }
            return false;
        }
    }
    return false;
} 

fn move_one(files: &mut Vec<(Option<usize>, i64)>, first_empty_idx: usize, last_full_idx: usize) -> (usize, usize) {

    let mut empty = files[first_empty_idx];
    let mut full = files[last_full_idx];
    if empty.1 == full.1 {
        files[first_empty_idx] = full;
        files.remove(last_full_idx);
        return (first_empty_idx, last_full_idx);
    } else if empty.1 > full.1 {
        files.remove(last_full_idx);
        files.insert(first_empty_idx, full);
        empty = (None, empty.1 - full.1);
        files[first_empty_idx + 1] = empty;
        return (first_empty_idx + 1, last_full_idx);
    } else if empty.1 < full.1 {
        let inserted_full = (full.0, empty.1);
        full = (full.0, full.1 - empty.1);
        files[first_empty_idx] = inserted_full;
        files[last_full_idx] = full;
        return (first_empty_idx, last_full_idx);
    }
    return (0, 0);
}

fn calc_checksum(files: &Vec<(Option<usize>, i64)>) -> i64 {
    files.iter().fold((0, 0), |(idx, sum), (id, size)| {
        if id.is_none() { (idx + size, sum) } else {
        (idx + size, sum + id.unwrap() as i64 * (gauss(idx - 1 + size) - gauss(idx - 1)))
    }}).1
}

fn gauss(n: i64) -> i64 {
    n*(n+1) / 2
}