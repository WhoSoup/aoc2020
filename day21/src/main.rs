use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref LINE: Regex = Regex::new(r"([\w\s]+) \(contains ([\w\s,]+)\)").unwrap();
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ingredients = HashSet::new();
    let mut not_safe = HashSet::new();
    let mut recipes: Vec<Vec<&str>> = vec![];

    content.lines().for_each(|line| {
        if let Some(caps) = LINE.captures(line) {
            let possible: Vec<&str> = caps
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.trim())
                .collect();

            recipes.push(possible.clone());

            for i in possible.clone() {
                ingredients.insert(i);
            }

            let allergens: Vec<&str> = caps
                .get(2)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.trim())
                .collect();

            for allergen in allergens {
                let entry = map.entry(allergen).or_insert(possible.clone());
                *entry = entry
                    .iter()
                    .filter(|x| possible.contains(*x))
                    .map(|a| *a)
                    .collect();
            }
        }
    });

    let mut sortable: Vec<&str> = vec![];

    for (_, v) in map {
        for i in v {
            not_safe.insert(i);
        }
    }

    for k in &not_safe {
        sortable.push(*k);
    }

    let safe: HashSet<&str> = ingredients
        .iter()
        .filter(|x| !not_safe.contains(**x))
        .map(|x| *x)
        .collect();

    let p1: i32 = recipes
        .iter()
        .map(|r| r.iter().fold(0, |a, x| a + safe.contains(x) as i32))
        .sum();

    println!("Part One: {:?}", p1);

    sortable.sort();
    println!("Part Two: {:?}", sortable.join(","));
}
