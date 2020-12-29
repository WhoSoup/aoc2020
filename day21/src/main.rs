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

    for (_, v) in &map {
        for i in v {
            not_safe.insert(*i);
        }
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

    let mut sortable: Vec<(&str, &str)> = vec![];
    while !map.is_empty() {
        map.clone().iter().for_each(|(k, v)| {
            if v.len() == 1 {
                let rest = v[0];
                sortable.push((k, rest));
                map.remove(k);

                map.iter_mut().for_each(|(_, v)| {
                    if let Some(index) = v.iter().position(|x| *x == rest) {
                        v.remove(index);
                    }
                });
            }
        });
    }

    sortable.sort();
    println!(
        "Part Two: {}",
        sortable
            .iter()
            .map(|x| x.1)
            .collect::<Vec<&str>>()
            .join(",")
    );
}
