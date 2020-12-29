const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: i32,
    sides: Vec<Side>,
}

#[allow(dead_code)]
impl Tile {
    pub fn new(data: &str) -> Tile {
        let mut spl = data.trim().split(":");
        let id: i32 = spl.next().unwrap().parse().unwrap();
        let data: Vec<Vec<bool>> = spl
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        let mut sides = vec![];
        for side in 0..4usize {
            sides.push(Tile::side(&data, side));
        }

        Tile { id, sides }
    }

    pub fn fits(&self, other: &Self) -> bool {
        let mut c = other.clone();
        for s in &self.sides {
            for o in c.sides.iter_mut() {
                if s.equal(o) {
                    return true;
                }
                o.flip();
                if s.equal(o) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn locked(&self) -> bool {
        self.sides.iter().any(|s| s.occupied)
    }

    pub fn rotate(&mut self) {
        let f = self.sides.remove(0);
        self.sides.push(f);
    }

    pub fn flip(&mut self) {
        for s in self.sides.iter_mut() {
            s.flip();
        }
    }

    fn side(raw: &Vec<Vec<bool>>, side: usize) -> Side {
        let mut grid = [false; 10];
        match side {
            LEFT => {
                for row in 0..10 {
                    grid[row] = raw[row][0];
                }
            }
            RIGHT => {
                for row in 0..10 {
                    grid[row] = raw[row][9];
                }
            }
            TOP => {
                for col in 0..10 {
                    grid[col] = raw[0][col];
                }
            }
            BOTTOM => {
                for col in 0..10 {
                    grid[col] = raw[9][col];
                }
            }

            _ => unreachable!(),
        }
        Side {
            occupied: false,
            grid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Side {
    occupied: bool,
    grid: [bool; 10],
}

#[allow(dead_code)]
impl Side {
    pub fn equal(&self, other: &Self) -> bool {
        self.grid.eq(&other.grid)
    }

    pub fn flipped(&self) -> Self {
        let mut grid = self.grid.clone();
        grid.reverse();
        Side {
            occupied: self.occupied,
            grid,
        }
    }

    fn flip(&mut self) {
        self.grid.reverse();
    }
}
