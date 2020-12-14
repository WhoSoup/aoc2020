use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"^mask = ([10X]+)$").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

enum Rule {
    Mask(u64, u64),
    Mem(u64, u64),
}

fn parse_mask(s: &str) -> (u64, u64) {
    let mut ones = 0;
    let mut zeros = 0;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        ones <<= 1;
        zeros <<= 1;
        if c != '0' {
            zeros += 1;
        }
        if c == '1' {
            ones += 1;
        }
    }
    (ones, zeros)
}

fn parse_rules(s: &str) -> Rule {
    if let Some(cap) = MASK_RE.captures(s) {
        let (a, b) = parse_mask(cap.get(1).unwrap().as_str());
        Rule::Mask(a, b)
    } else if let Some(cap) = MEM_RE.captures(s) {
        Rule::Mem(
            cap.get(1).unwrap().as_str().parse().unwrap(),
            cap.get(2).unwrap().as_str().parse().unwrap(),
        )
    } else {
        panic!("bad line: {}", s);
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let rules: Vec<Rule> = content.lines().map(parse_rules).collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut zeromask = 0;
    let mut onesmask = 0;
    for r in rules {
        match r {
            Rule::Mask(o, z) => {
                onesmask = o;
                zeromask = z;
                //println!("ones: {:b} zeros: {:b}", o, z);
            }
            Rule::Mem(p, v) => {
                memory.insert(p, (v & zeromask) | onesmask);
            }
        }
    }

    let mut sum = 0;
    for (_, &v) in memory.iter() {
        sum += v;
    }

    println!("Part One: {}", sum);
}
