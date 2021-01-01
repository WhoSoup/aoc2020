fn find_loop_size(subject: u64, public: u64) -> u64 {
    let mut key = 1;
    let mut loop_size = 1;
    loop {
        key *= subject;
        key %= 20201227;

        if key == public {
            return loop_size;
        }

        loop_size += 1;
    }
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut key = 1;
    for _ in 0..loop_size {
        key *= subject;
        key %= 20201227;
    }
    key
}

fn main() {
    let card_key = 1614360;
    let card_loop = find_loop_size(7, card_key);

    let door_key = 7734663;
    let door_loop = find_loop_size(7, door_key);

    let answer_a = transform(card_key, door_loop);
    let answer_b = transform(door_key, card_loop);

    assert_eq!(answer_a, answer_b);

    println!("Part One: {}", answer_a);
}
