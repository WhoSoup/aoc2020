fn play_game(start: Vec<usize>, rounds: i32) -> Vec<usize> {
    let mut cups = vec![0; start.len()];
    let size = cups.len();

    for n in 0..start.len() {
        cups[start[n]] = start[(n + 1) % size];
    }

    let mut pos = start[0];

    for _r in 0..rounds {
        let a = cups[pos];
        let b = cups[a];
        let c = cups[b];

        cups[pos] = cups[c];

        let mut dest = (pos + size - 1) % size;
        while dest == a || dest == b || dest == c {
            dest = (dest + size - 1) % size;
        }

        cups[c] = cups[dest];
        cups[dest] = a;

        pos = cups[pos];
    }

    cups
}

fn part_one(cups: &str) {
    let cups: Vec<usize> = cups
        .chars()
        .map(|n| (n.to_digit(10).unwrap() - 1) as usize)
        .collect();

    let result = play_game(cups.clone().into_iter().collect(), 100);

    print!("Part One: ");
    let mut start = 0;
    for _ in 0..result.len() {
        if start != 0 {
            print!("{}", start + 1);
        }
        start = result[start];
    }
    println!();
}

#[allow(dead_code)]
fn part_two(cups: &str) {
    let mut cups: Vec<usize> = cups
        .chars()
        .map(|n| (n.to_digit(10).unwrap() - 1) as usize)
        .collect();

    for c in 9..1_000_000 {
        cups.push(c);
    }

    let result = play_game(cups, 10_000_000);

    let a = result[0];
    let b = result[a];

    println!("Part Two: {} * {} = {}", a + 1, b + 1, (a + 1) * (b + 1));
}

fn main() {
    part_one("974618352");
    part_two("974618352");
}
