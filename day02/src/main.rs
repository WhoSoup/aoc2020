use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w+): (\w+)$").unwrap();
}

fn valid_password(line: &str) -> bool {
    let cap = RE.captures(line).unwrap();
    let min: i32 = cap[1].parse().unwrap();
    let max: i32 = cap[2].parse().unwrap();
    let needle = cap[3].chars().next().unwrap();
    let haystack: Vec<char> = cap[4].chars().collect();

    let mut count: i32 = 0;
    for c in haystack {
        if c == needle {
            count += 1;
        }
    }

    count >= min && count <= max
}

fn valid_password2(line: &str) -> bool {
    let cap = RE.captures(line).unwrap();
    let one: usize = cap[1].parse().unwrap();
    let two: usize = cap[2].parse().unwrap();

    let needle = cap[3].as_bytes()[0];
    let haystack = cap[4].as_bytes().to_owned();
    

    (needle == haystack[one-1] || needle == haystack[two-1])
        && (haystack[one-1] != haystack[two-1])
}

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("can't read file");

    let mut total: i32 = 0;
    let mut total2: i32 = 0;
    for line in content.lines() {
        if valid_password(line) {
            total += 1;
        }
        if valid_password2(line) {
            total2 += 1;
        }
    }

    println!("Part One: {}", total);
    println!("Part Two: {}", total2);
}
