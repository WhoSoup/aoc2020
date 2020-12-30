use std::collections::VecDeque;

fn play_game(cups: VecDeque<usize>, rounds: i32) -> VecDeque<usize> {
    let size = cups.len();
    let mut cups = cups;

    for _turn in 0..rounds {
        /*println!(
            "{}: {:?}",
            turn,
            cups.iter().map(|n| n + 1).collect::<Vec<usize>>()
        );*/
        let a = cups.remove(1).unwrap();
        let b = cups.remove(1).unwrap();
        let c = cups.remove(1).unwrap();

        let mut dest = (cups[0] + size - 1) % size;
        //        println!("cups = {:?}, dest = {}", cups, dest);
        while dest == a || dest == b || dest == c {
            dest = (dest + size - 1) % size;
        }

        let dest_index = cups.iter().position(|&n| n == dest).unwrap() + 1;

        cups.insert(dest_index, c);
        cups.insert(dest_index, b);
        cups.insert(dest_index, a);

        cups.rotate_left(1);
    }

    let pos = cups.iter().position(|&n| n == 0).unwrap();
    cups.rotate_left(pos);

    cups
}

fn part_one(cups: &str) {
    let cups: VecDeque<usize> = cups
        .chars()
        .map(|n| (n.to_digit(10).unwrap() - 1) as usize)
        .collect();

    let result = play_game(cups, 100);

    println!(
        "Part One: {}",
        result
            .into_iter()
            .skip(1)
            .map(|n| (n + 1).to_string())
            .collect::<Vec<String>>()
            .join(""),
    );
}

#[allow(dead_code)]
fn part_two(cups: &str) {
    let mut cups: VecDeque<usize> = cups
        .chars()
        .map(|n| (n.to_digit(10).unwrap() - 1) as usize)
        .collect();

    for c in 9..10_000_000 {
        cups.push_back(c);
    }

    let result = play_game(cups, 1_000_000);

    println!(
        "Part Two: {} * {} = {}",
        result[1],
        result[2],
        result[1] * result[2],
    );
}

fn main() {
    part_one("974618352");
    //part_two("974618352");
}
