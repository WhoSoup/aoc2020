mod tile;
use itertools::Itertools;
use tile::*;

fn parse_tiles(content: String) -> Vec<Tile> {
    content
        .split("Tile")
        .filter(|l| l.len() > 0)
        .map(|tile| Tile::new(tile))
        .collect()
}
fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let tiles = parse_tiles(content);

    let mut matches = std::collections::HashMap::<i32, i32>::new();
    for pair in tiles.iter().combinations(2) {
        if pair[0].fits(pair[1]) {
            matches
                .entry(pair[0].id)
                .and_modify(|i| *i += 1)
                .or_insert(1);
            matches
                .entry(pair[1].id)
                .and_modify(|i| *i += 1)
                .or_insert(1);
        }
    }

    let mut res = 1u64;
    for (k, v) in matches {
        if v == 2 {
            res *= k as u64;
        }
    }
    println!("Part One: {}", res);
}
