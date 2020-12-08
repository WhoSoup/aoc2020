use std::collections::HashSet;
#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jump(i32),
    Noop(i32),
}

#[derive(Debug)]
enum AppError {
    BadJump,
    Overflow,
    Loop,
}

struct BootApp {
    instructions: Vec<Instruction>,
}

impl BootApp {
    fn run(&self, change: usize) -> Result<i32, AppError> {
        let mut pos: usize = 0;
        let mut acc: i32 = 0;

        let mut visited = HashSet::<usize>::new();

        loop {
            if pos < self.instructions.len() {
                if visited.contains(&pos) {
                    if change == 0 {
                        println!("Part One: {}", acc);
                    }
                    return Err(AppError::Loop);
                }
                visited.insert(pos);

                let mut ins = self.instructions[pos];
                if pos == change {
                    // swap
                    ins = match ins {
                        Instruction::Jump(x) => Instruction::Noop(x),
                        Instruction::Noop(x) => Instruction::Jump(x),
                        _ => ins,
                    }
                }

                match ins {
                    Instruction::Acc(x) => {
                        match acc.checked_add(x) {
                            Some(x) => acc = x,
                            None => return Err(AppError::Overflow),
                        }
                        pos += 1;
                    }
                    Instruction::Jump(x) => pos = ((pos as i32) + x) as usize,
                    Instruction::Noop(_) => pos += 1,
                }
            } else if pos == self.instructions.len() {
                return Ok(acc);
            } else {
                return Err(AppError::BadJump);
            }
        }
    }
}

fn parse_instruction(s: &str) -> Instruction {
    match &s[0..3] {
        "acc" => Instruction::Acc(s[4..].parse().unwrap()),
        "jmp" => Instruction::Jump(s[4..].parse().unwrap()),
        "nop" => Instruction::Noop(s[4..].parse().unwrap()),
        _ => panic!("invalid parse"),
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<Instruction> = content
        .trim()
        .lines()
        .map(|s| parse_instruction(s))
        .collect();

    let total = instructions.len();

    let ba = BootApp { instructions };

    for i in 0..total {
        match ba.run(i) {
            Ok(x) => {
                println!("Part Two: {}", x);
                break;
            }
            Err(_) => (),
        }
    }
}
