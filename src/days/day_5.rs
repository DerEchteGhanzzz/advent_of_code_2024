use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve_a(input: &Vec<String>) -> i64 {
    let split = input.split(|n| n == "").collect::<Vec<_>>();
    let updates = split[1].iter().map(|s| s.split(",").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>());
    let relations = split[0];

    let precedence_book = parse_relations(relations);
    updates.filter(|s| check_update(s, &precedence_book)).map(|update| update[update.len() / 2]).sum::<i32>() as i64
}

pub fn solve_b(input: &Vec<String>) -> i64 {
    let split = input.split(|n| n == "").collect::<Vec<_>>();
    let updates = split[1].iter().map(|s| s.split(",").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>());
    let relations = split[0];

    let precedence_book = parse_relations(relations);
    let wrongs = updates.filter(|s| !check_update(s, &precedence_book));

    wrongs.map(|update| update.iter().map(|page| Page::new(*page, &precedence_book)).sorted().map(|p| p.value).collect::<Vec<_>>()).map(|update| update[update.len() / 2]).sum::<i32>() as i64
}

fn check_update(update: &Vec<i32>, relation_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut after = HashSet::new();
    for page in update.iter().rev() {
        if relation_map.contains_key(page) && after.intersection(relation_map.get(page).unwrap()).count() > 0 {
            return false;
        }
        after.insert(*page);
    }
    true
}

fn parse_relations(relations: &[String]) -> HashMap<i32, HashSet<i32>> {
    let mut relation_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    
    for relation in relations {
        let split = relation.split("|").collect::<Vec<_>>();
        let before = split[0].parse::<i32>().unwrap();
        let after = split[1].parse::<i32>().unwrap();

        if relation_map.contains_key(&after) {
            relation_map.get_mut(&after).unwrap().insert(before);
        } else {
            relation_map.insert(after, HashSet::from([before]));
        }
    }
    relation_map
}

struct Page {
    value: i32,
    before: HashSet<i32>,
}

impl Page {
    pub fn new(value: i32, relations: &HashMap<i32, HashSet<i32>>) -> Self {
        let r = relations.get(&value).unwrap().clone();
        Page {value, before: r}
    } 

}

impl Eq for Page {
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.before.contains(&other.value) {
            return std::cmp::Ordering::Greater
        }
        std::cmp::Ordering::Less
    }
}

impl PartialOrd for Page {
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.before.contains(&other.value) {
            return Some(std::cmp::Ordering::Greater)
        }
        Some(std::cmp::Ordering::Less)
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}