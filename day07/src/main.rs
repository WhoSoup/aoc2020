use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref OUTERRE: Regex = Regex::new(r"^(\w+ \w+) bag").unwrap();
    static ref INNERRE: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
}

#[derive(Debug)]
struct Bag {
    quantity: u32,
    color: String,
}

fn parse_bag(s: &str) -> (Bag, Vec<Bag>) {
    let outer = OUTERRE.captures(s).unwrap();
    let inner = INNERRE.captures_iter(s);

    let mut v = vec![];
    for b in inner {
        v.push(Bag {
            quantity: b.get(1).unwrap().as_str().parse().unwrap(),
            color: String::from(b.get(2).unwrap().as_str()),
        })
    }

    (
        Bag {
            quantity: 1,
            color: String::from(outer.get(1).unwrap().as_str()),
        },
        v,
    )
}

fn bag_count(rules: &Vec<(Bag, Vec<Bag>)>, bag: &str) -> u32 {
    for r in rules {
        if r.0.color.eq(bag) {
            let mut sum = 1;
            for b in &r.1 {
                sum += b.quantity * bag_count(rules, &b.color);
            }
            return sum;
        }
    }
    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rules: Vec<(Bag, Vec<Bag>)> = input.trim().lines().map(|x| parse_bag(x)).collect();

    let mut can_hold_gold = HashSet::<&str>::new();
    can_hold_gold.insert("shiny gold"); // initial, doesn't count for total

    loop {
        let mut found = 0;
        for (outer, inner) in &rules {
            for i in inner {
                if can_hold_gold.contains(&i.color[..]) && !can_hold_gold.contains(&outer.color[..])
                {
                    can_hold_gold.insert(&outer.color[..]);
                    found += 1;
                    break;
                }
            }
        }

        if found == 0 {
            break;
        }
    }

    println!("Part One: {}", can_hold_gold.len() - 1); // subtract initial
    println!("Part Two: {}", bag_count(&rules, "shiny gold") - 1);
}
