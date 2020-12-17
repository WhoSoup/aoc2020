use std::collections::HashSet;

struct Dimension4 {
    active: HashSet<(i32, i32, i32, i32)>,
}

impl Dimension4 {
    fn new() -> Dimension4 {
        Dimension4 {
            active: HashSet::new(),
        }
    }

    fn init(&mut self, content: String) {
        for (row, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.active.insert((row as i32, col as i32, 0, 0));
                }
            }
        }
    }

    fn round(&mut self) {
        let mut new = HashSet::new();
        let mut visited = HashSet::new();

        for active_point in &self.active {
            for point in neighbors4(*active_point) {
                if visited.contains(&point) {
                    continue;
                }
                visited.insert(point);

                let count: i32 = neighbors4(point)
                    .iter()
                    .map(|x| self.active.contains(x) as i32)
                    .sum();
                if self.active.contains(&point) {
                    if count == 2 || count == 3 {
                        new.insert(point);
                    }
                } else {
                    if count == 3 {
                        new.insert(point);
                    }
                }
            }
        }

        self.active = new;
    }
}

fn neighbors4(point: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    v.push((point.0 + x, point.1 + y, point.2 + z, point.3 + w));
                }
            }
        }
    }
    v
}

struct Dimension3 {
    active: HashSet<(i32, i32, i32)>,
}

impl Dimension3 {
    fn new() -> Dimension3 {
        Dimension3 {
            active: HashSet::new(),
        }
    }

    fn init(&mut self, content: String) {
        for (row, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.active.insert((row as i32, col as i32, 0));
                }
            }
        }
    }

    fn round(&mut self) {
        let mut new = HashSet::new();
        let mut visited = HashSet::new();

        for active_point in &self.active {
            for point in neighbors3(*active_point) {
                if visited.contains(&point) {
                    continue;
                }
                visited.insert(point);

                let count: i32 = neighbors3(point)
                    .iter()
                    .map(|x| self.active.contains(x) as i32)
                    .sum();
                if self.active.contains(&point) {
                    if count == 2 || count == 3 {
                        new.insert(point);
                    }
                } else {
                    if count == 3 {
                        new.insert(point);
                    }
                }
            }
        }

        self.active = new;
    }
}

fn neighbors3(point: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                v.push((point.0 + x, point.1 + y, point.2 + z));
            }
        }
    }
    v
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut dim3 = Dimension3::new();
    dim3.init(content.clone());

    let mut dim4 = Dimension4::new();
    dim4.init(content.clone());

    for _ in 0..6 {
        dim3.round();
        dim4.round();
    }

    println!("Part One: {}", dim3.active.len());
    println!("Part Two: {}", dim4.active.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neighbors3() {
        assert!(neighbors3((1, 2, 3)).len() == 26);
        assert_eq!(
            neighbors3((0, 0, 0)),
            vec![
                (-1, -1, -1),
                (-1, -1, 0),
                (-1, -1, 1),
                (-1, 0, -1),
                (-1, 0, 0),
                (-1, 0, 1),
                (-1, 1, -1),
                (-1, 1, 0),
                (-1, 1, 1),
                (0, -1, -1),
                (0, -1, 0),
                (0, -1, 1),
                (0, 0, -1),
                (0, 0, 1),
                (0, 1, -1),
                (0, 1, 0),
                (0, 1, 1),
                (1, -1, -1),
                (1, -1, 0),
                (1, -1, 1),
                (1, 0, -1),
                (1, 0, 0),
                (1, 0, 1),
                (1, 1, -1),
                (1, 1, 0),
                (1, 1, 1)
            ]
        )
    }
}
