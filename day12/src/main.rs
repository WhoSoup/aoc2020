const EAST: Point = Point { x: 1, y: 0 };
const ZERO: Point = Point { x: 0, y: 0 };

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

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn turn_left(&mut self) {
        self.x *= -1;
        self.swap();
    }
    fn turn_right(&mut self) {
        self.y *= -1;
        self.swap();
    }

    fn swap(&mut self) {
        *self = Point {
            x: self.y,
            y: self.x,
        }
    }

    fn manhattan(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, other: i32) {
        self.x *= other;
        self.y *= other;
    }
}

struct Turtle {
    pos: Point,
    dir: Point,
    instructions: Vec<Instruction>,
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

impl Turtle {
    fn new(instructions: Vec<Instruction>) -> Turtle {
        Turtle {
            pos: ZERO,
            dir: EAST,
            instructions,
        }
    }

    fn part_one(&mut self) -> i32 {
        for ins in &self.instructions {
            match ins {
                Instruction::NORTH(v) => self.pos.y -= v,
                Instruction::SOUTH(v) => self.pos.y += v,
                Instruction::EAST(v) => self.pos.x += v,
                Instruction::WEST(v) => self.pos.x -= v,
                Instruction::FORWARD(v) => {
                    self.pos += self.dir * *v;
                }
                Instruction::LEFT(mut v) => {
                    while v > 0 {
                        self.dir.turn_left();
                        v -= 90;
                    }
                }
                Instruction::RIGHT(mut v) => {
                    while v > 0 {
                        self.dir.turn_right();
                        v -= 90;
                    }
                }
            }
        }

        self.pos.manhattan()
    }

    fn part_two(&mut self) -> i32 {
        self.dir = Point { x: 10, y: -1 };

        for ins in &self.instructions {
            match ins {
                Instruction::NORTH(v) => self.dir.y -= v,
                Instruction::SOUTH(v) => self.dir.y += v,
                Instruction::EAST(v) => self.dir.x += v,
                Instruction::WEST(v) => self.dir.x -= v,
                Instruction::FORWARD(v) => {
                    self.pos += self.dir * *v;
                }
                Instruction::LEFT(mut v) => {
                    while v > 0 {
                        self.dir.turn_left();
                        v -= 90;
                    }
                }
                Instruction::RIGHT(mut v) => {
                    while v > 0 {
                        self.dir.turn_right();
                        v -= 90;
                    }
                }
            }
        }

        self.pos.manhattan()
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<Instruction> = content.lines().map(parse_instruction).collect();

    let mut p1 = Turtle::new(data.clone());
    let mut p2 = Turtle::new(data);

    println!("Part One: {}", p1.part_one());
    println!("Part Two: {}", p2.part_two());
}
