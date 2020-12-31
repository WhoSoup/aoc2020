mod tile;

use tile::*;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let lines: Vec<Vec<Direction>> = content.lines().map(|l| Direction::parse_line(l)).collect();
    let mut t = Grid::new();

    for line in lines {
        t.travel(line);
    }

    println!("Part One: {}", t.count());

    /*    for i in 1..15 {
        t = t.next_day();
        println!("Day {}: {}", i, t.count())
    }*/
}
