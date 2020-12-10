fn p1(data: Vec<i32>) {
    let mut jolt = 0;
    let mut diffs = vec![0; 4];
    for n in data {
        diffs[(n - jolt) as usize] += 1;
        jolt = n;
    }
    diffs[3] += 1; // final adapter

    println!("Part One: {} ", diffs[1] * diffs[3]);
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut adapters: Vec<i32> = content
        .trim()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    adapters.sort();
    p1(adapters.clone());
}
