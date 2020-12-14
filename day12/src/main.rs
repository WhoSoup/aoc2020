const DIR_NORTH: (i32, i32) = (0, -1);
const DIR_SOUTH: (i32, i32) = (0, 1);
const DIR_EAST: (i32, i32) = (1, 0);
const DIR_WEST: (i32, i32) = (-1, 0);

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NORTH(i32),
    SOUTH(i32),
    EAST(i32),
    WEST(i32),
    LEFT(i32),
    RIGHT(i32),
    FORWARD(i32),
}

fn parse_instruction(ins: &str) -> Instruction {
    let v: i32 = ins[1..].parse().unwrap();
    match ins.chars().next().unwrap() {
        'N' => Instruction::NORTH(v),
        'S' => Instruction::SOUTH(v),
        'E' => Instruction::EAST(v),
        'W' => Instruction::WEST(v),
        'L' => Instruction::LEFT(v),
        'R' => Instruction::RIGHT(v),
        'F' => Instruction::FORWARD(v),
        _ => panic!("bad parse"),
    }
}

fn part_one(instructions: Vec<Instruction>) -> i32 {
    let mut pos = (0, 0);
    let mut dir = DIR_EAST;

    for ins in instructions {
        match ins {
            Instruction::NORTH(v) => pos.1 -= v,
            Instruction::SOUTH(v) => pos.1 += v,
            Instruction::EAST(v) => pos.0 += v,
            Instruction::WEST(v) => pos.0 -= v,
            Instruction::FORWARD(v) => {
                pos.0 += dir.0 * v;
                pos.1 += dir.1 * v;
            }
            Instruction::LEFT(mut v) => {
                while v > 0 {
                    dir = match dir {
                        DIR_NORTH => DIR_WEST,
                        DIR_WEST => DIR_SOUTH,
                        DIR_SOUTH => DIR_EAST,
                        _ => DIR_NORTH,
                    };
                    v -= 90;
                }
            }
            Instruction::RIGHT(mut v) => {
                while v > 0 {
                    dir = match dir {
                        DIR_NORTH => DIR_EAST,
                        DIR_EAST => DIR_SOUTH,
                        DIR_SOUTH => DIR_WEST,
                        _ => DIR_NORTH,
                    };
                    v -= 90;
                }
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part_two(instructions: Vec<Instruction>) -> i32 {
    let mut pos = (0, 0);
    let mut waypoint = (10, -1);

    for ins in instructions {
        match ins {
            Instruction::NORTH(v) => waypoint.1 -= v,
            Instruction::SOUTH(v) => waypoint.1 += v,
            Instruction::EAST(v) => waypoint.0 += v,
            Instruction::WEST(v) => waypoint.0 -= v,
            Instruction::FORWARD(v) => {
                pos.0 += v * waypoint.0;
                pos.1 += v * waypoint.1;
            }
            Instruction::LEFT(mut v) => {
                while v > 0 {
                    let mut tmp = waypoint;
                    tmp.0 *= -1;
                    waypoint.0 = tmp.1;
                    waypoint.1 = tmp.0;
                    v -= 90;
                }
            }
            Instruction::RIGHT(mut v) => {
                while v > 0 {
                    let mut tmp = waypoint;
                    tmp.1 *= -1;
                    waypoint.0 = tmp.1;
                    waypoint.1 = tmp.0;
                    v -= 90;
                }
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<Instruction> = content.lines().map(parse_instruction).collect();

    println!("Part One: {}", part_one(data.clone()));
    println!("Part Two: {}", part_two(data));
}
