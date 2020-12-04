use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEIGHT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    static ref HAIR: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref EYE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn valid_fields(pp: &HashMap<&str, &str>) -> bool {
    REQUIRED.iter().fold(true, |acc, x| acc && pp.contains_key(x))
}

fn valid_passport(pp: &HashMap<&str, &str>) -> bool {
    pp.iter().fold(true, |acc, (k,v)| {
        acc && valid_fields(pp) && match k.clone() {
            "byr" => match v.parse::<u32>() {
                Ok(num) => num >= 1920 && num <= 2002,
                Err(_) => false
            },
            "iyr" => match v.parse::<u32>() {
                Ok(num) => num >= 2010 && num <= 2020,
                Err(_) => false
            },
            "eyr" => match v.parse::<u32>() {
                Ok(num) => num >= 2020 && num <= 2030,
                Err(_) => false
            },
            "hgt" => {
                let cap = HEIGHT.captures(v);
                match cap {
                    Some(x) => {
                        let num: i32 = x[1].parse().unwrap();
                        (x[2].eq("cm") && num >= 150 && num <= 193)
                        || (x[2].eq("in") && num >= 59 && num <= 76)
                    },
                    None => false
                }
            },
            "hcl" => HAIR.is_match(v),
            "ecl" => EYE.is_match(v),
            "pid" => PID.is_match(v),
            _ => true
        }
    })
}

fn main() {
    let raw = std::fs::read_to_string("input.txt").unwrap()
        .replace("\n\n", "@")
        .replace("\n", " ")
        .replace("@", "\n");
    let content = raw.lines()
        .map(|x| 
            x.trim()
                .split(" ")
                .fold(HashMap::new(), |mut acc, x| {
                    let split: Vec<&str> = x.split(":").collect();
                    acc.insert(split[0], split[1]);
                    acc
                })
                );

    println!("Part One: {}", content.clone().filter(|x| valid_fields(x)).count());
    println!("Part Two: {}", content.clone().filter(|x| valid_passport(x)).count());
}
