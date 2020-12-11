const SURROUNDING: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Seat {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

struct Field {
    data: Vec<Vec<Seat>>,
    width: i32,
    height: i32,
}

impl Field {
    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn count(&self, x: i32, y: i32) -> i32 {
        let mut sum = 0;
        for (i, j) in SURROUNDING.iter() {
            if self.is_valid(x + i, y + j) {
                sum += if self.data[(y + j) as usize][(x + i) as usize] == Seat::OCCUPIED {
                    1
                } else {
                    0
                }
            }
        }
        sum
    }

    fn count_los(&self, x: i32, y: i32) -> i32 {
        let mut sum = 0;
        for (i, j) in SURROUNDING.iter() {
            let mut xx = x;
            let mut yy = y;
            loop {
                xx += i;
                yy += j;
                if self.is_valid(xx, yy) {
                    match self.data[yy as usize][xx as usize] {
                        Seat::OCCUPIED => {
                            sum += 1;
                            break;
                        }
                        Seat::EMPTY => {
                            break;
                        }
                        Seat::FLOOR => {}
                    }
                } else {
                    break;
                }
            }
        }
        sum
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.data {
            for seat in row {
                print!(
                    "{}",
                    match *seat {
                        Seat::OCCUPIED => '#',
                        Seat::EMPTY => 'L',
                        Seat::FLOOR => '.',
                    }
                );
            }
            println!();
        }
        println!();
    }

    fn find_equilibrium(&mut self, threshold: i32, cnt: fn(&Self, i32, i32) -> i32) -> i32 {
        let mut changes = true;

        while changes {
            changes = false;
            let mut cp = self.data.clone();

            for x in 0..self.width {
                for y in 0..self.height {
                    match self.data[y as usize][x as usize] {
                        Seat::EMPTY => {
                            if cnt(self, x, y) == 0 {
                                changes = true;
                                cp[y as usize][x as usize] = Seat::OCCUPIED;
                            }
                        }
                        Seat::OCCUPIED => {
                            if cnt(self, x, y) >= threshold {
                                changes = true;
                                cp[y as usize][x as usize] = Seat::EMPTY;
                            }
                        }
                        Seat::FLOOR => {}
                    }
                }
            }

            self.data = cp;
        }
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|seat| if *seat == Seat::OCCUPIED { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum()
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<Vec<Seat>> = content
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '#' => Seat::OCCUPIED,
                    'L' => Seat::EMPTY,
                    _ => Seat::FLOOR,
                })
                .collect()
        })
        .collect();

    let mut p1 = Field {
        height: data.len() as i32,
        width: data[0].len() as i32,
        data: data.clone(),
    };

    let mut p2 = Field {
        height: data.len() as i32,
        width: data[0].len() as i32,
        data,
    };

    println!("Part One: {}", p1.find_equilibrium(4, Field::count));
    println!("Part Two: {}", p2.find_equilibrium(5, Field::count_los));
}
