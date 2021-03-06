use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    pub fn parse_line(line: &str) -> Vec<Direction> {
        line.replace("se", "a")
            .replace("sw", "b")
            .replace("nw", "c")
            .replace("ne", "d")
            .chars()
            .map(|c| match c {
                'e' => Direction::E,
                'w' => Direction::W,
                'a' => Direction::SE,
                'b' => Direction::SW,
                'c' => Direction::NW,
                'd' => Direction::NE,
                _ => unreachable!(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    q: i32,
    r: i32,
}

impl Coord {
    pub fn new(q: i32, r: i32) -> Coord {
        Coord { q, r }
    }

    pub fn travel(&mut self, dir: Direction) {
        self.add(Coord::dir_to_vec(dir));
    }

    fn add(&mut self, (q, r): (i32, i32)) {
        self.q += q;
        self.r += r;
    }

    fn dir_to_vec(dir: Direction) -> (i32, i32) {
        match dir {
            Direction::E => (1, 0),
            Direction::SE => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (0, -1),
            Direction::NE => (1, -1),
        }
    }

    pub fn adjacent(&self) -> Vec<Coord> {
        vec![
            self.clone_add(Direction::E),
            self.clone_add(Direction::SE),
            self.clone_add(Direction::SW),
            self.clone_add(Direction::W),
            self.clone_add(Direction::NW),
            self.clone_add(Direction::NE),
        ]
    }

    fn clone_add(self, dir: Direction) -> Coord {
        let (q, r) = Coord::dir_to_vec(dir);
        Coord {
            q: self.q + q,
            r: self.r + r,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    tiles: HashSet<Coord>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            tiles: HashSet::new(),
        }
    }

    pub fn travel(&mut self, steps: Vec<Direction>) {
        let mut pos = Coord::new(0, 0);
        for step in steps {
            pos.travel(step);
        }

        if !self.tiles.insert(pos) {
            self.tiles.remove(&pos);
        }
    }

    pub fn count(&self) -> usize {
        self.tiles.len()
    }

    pub fn next_day(&self) -> Grid {
        let mut g = Grid::new();

        let count = |coord: &Coord| -> i32 {
            coord
                .adjacent()
                .iter()
                .map(|c| self.tiles.contains(c) as i32)
                .sum()
        };

        for t in &self.tiles {
            if count(t) == 1 {
                println!("count {:?} = 1", t);
                g.tiles.insert(*t);
            }

            for n in t.adjacent() {
                if !self.tiles.contains(&n) && count(&n) == 2 {
                    println!("white tile count {:?} = 2 (ajc to {:?})", n, t);
                    // is white & 2 adj
                    g.tiles.insert(n);
                }
            }
        }

        g
    }

    pub fn print(&self) {
        for r in -5..=5 {
            if (r + 1) % 2 == 0 {
                print!("      ");
            }
            for q in -5..=5 {
                if self.tiles.contains(&Coord::new(q, r)) {
                    print!("    XXX    ");
                } else {
                    print!(" ({: ^3},{: ^3}) ", q, r);
                }
            }
            println!();
            println!();
        }
        println!();
    }
}
