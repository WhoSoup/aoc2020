use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

lazy_static! {
    static ref RSEQ: Regex = Regex::new(r"^(\d+): ([\d\s]+)$").unwrap();
    static ref ROR: Regex = Regex::new(r"^(\d+): ([\d\s]+) \| ([\d\s]+)$").unwrap();
    static ref RCHAR: Regex = Regex::new(r###"^(\d+): "(\w)"$"###).unwrap();
    static ref RULESET: HashMap::<i32, Box<dyn Rule>> = {
        let content = std::fs::read_to_string("rules.txt").unwrap();
        parse(content)
    };
}
trait Rule: std::fmt::Debug + std::marker::Sync {
    fn validate<'a>(&self, line: &'a str) -> (bool, &'a str);
}

#[derive(Debug)]
struct Char {
    char: char,
}

impl Rule for Char {
    fn validate<'a>(&self, line: &'a str) -> (bool, &'a str) {
        if line.len() > 0 {
            (line.chars().next().unwrap() == self.char, &line[1..])
        } else {
            (false, line)
        }
    }
}
#[derive(Debug)]
struct Or {
    a: Seq,
    b: Seq,
}

impl Rule for Or {
    fn validate<'a>(&self, line: &'a str) -> (bool, &'a str) {
        let a = self.a.validate(line);
        if a.0 {
            a
        } else {
            self.b.validate(line)
        }
    }
}
#[derive(Debug)]
struct Seq {
    rules: Vec<i32>,
}

impl Rule for Seq {
    fn validate<'a>(&self, line: &'a str) -> (bool, &'a str) {
        //println!("checking Seq: {:?}", self.rules);
        let mut rest = line;
        for r in &self.rules {
            let rule = RULESET.get(r).unwrap();
            let res = rule.validate(rest);
            if !res.0 {
                //println!("false: \"{}\" doesn't match rule {}: {:?}", rest, r, rule);
                return (false, line);
            } else {
                //println!("true: \"{}\" matches rule {}: {:?}", rest, r, rule);
            }
            rest = res.1;
        }
        return (true, rest);
    }
}

fn parse(s: String) -> HashMap<i32, Box<dyn Rule>> {
    let mut ruleset: HashMap<i32, Box<dyn Rule>> = HashMap::new();

    let get_id = |x: &Captures| x.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let parse_seq = |x: &Captures, group: usize| {
        x.get(group)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>()
    };

    s.lines().for_each(|line| {
        if let Some(caps) = RSEQ.captures(line) {
            ruleset.insert(
                get_id(&caps),
                Box::new(Seq {
                    rules: parse_seq(&caps, 2),
                }),
            );
        } else if let Some(caps) = ROR.captures(line) {
            ruleset.insert(
                get_id(&caps),
                Box::new(Or {
                    a: Seq {
                        rules: parse_seq(&caps, 2),
                    },
                    b: Seq {
                        rules: parse_seq(&caps, 3),
                    },
                }),
            );
        } else if let Some(caps) = RCHAR.captures(line) {
            ruleset.insert(
                get_id(&caps),
                Box::new(Char {
                    char: caps.get(2).unwrap().as_str().chars().next().unwrap(),
                }),
            );
        } else {
            unreachable!();
        }
    });
    ruleset
}

fn main() {
    //println!("{:?}", RULESET);

    let content = std::fs::read_to_string("input.txt").unwrap();
    let ok = |(a, b): (bool, &str)| a && b.len() == 0;

    println!(
        "Part One: {:?}",
        content
            .lines()
            .map(|line| { ok(RULESET.get(&0).unwrap().validate(line)) as i32 })
            .sum::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rule_char() {
        let r = Char { char: 'a' };

        assert_eq!(r.validate("aab"), (true, "ab"));
        assert_eq!(r.validate("baab"), (false, "aab"));
        assert_eq!(r.validate(""), (false, ""));
    }
}
