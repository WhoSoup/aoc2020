use lazy_static::lazy_static;
use regex::Regex;
#[derive(Debug)]
enum Op {
    Number(u64),
    Plus,
    Mul,
}

lazy_static! {
    static ref PARENTHESES: Regex = Regex::new(r"\(([^\(\)]+)\)").unwrap();
}

fn convert(s: &str) -> Op {
    match s {
        "*" => Op::Mul,
        "+" => Op::Plus,
        _ => Op::Number(s.parse().unwrap()),
    }
}

fn calculate(exp: &str) -> u64 {
    let mut stack: Vec<Op> = exp.split(" ").map(convert).collect();
    stack.reverse();

    let mut total = match stack.pop().unwrap() {
        Op::Number(n) => n,
        _ => unreachable!(),
    };

    while stack.len() > 1 {
        let op = stack.pop().unwrap();
        let next = match stack.pop().unwrap() {
            Op::Number(n) => n,
            _ => unreachable!(),
        };

        total = match op {
            Op::Mul => total * next,
            Op::Plus => total + next,
            _ => unreachable!(),
        }
    }

    total
}

fn evaluate_expression(line: &str) -> u64 {
    let mut expression = line.to_string();

    while let Some(cap) = PARENTHESES.captures(&expression) {
        let result = calculate(cap.get(1).unwrap().as_str());
        expression = expression.replace(cap.get(0).unwrap().as_str(), &result.to_string());
    }
    calculate(&expression)
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "Part One: {}",
        content.lines().map(evaluate_expression).sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate_expression("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate_expression("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate_expression("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            evaluate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            evaluate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
