fn p1(data: Vec<u64>) -> u64 {
    let mut jolt = 0;
    let mut diffs = vec![0; 4];
    for n in data {
        diffs[(n - jolt) as usize] += 1;
        jolt = n;
    }
    diffs[1] * (diffs[3] + 1) // final adapter
}

fn p2(data: Vec<u64>) -> u64 {
    let mut data = data;
    data.reverse();
    data.push(0); // starting point

    let target = data[0] + 3;
    let mut reach = vec![0 as u64; target as usize + 1];
    reach[target as usize] = 1;

    for n in data {
        for i in n + 1..=n + 3 {
            reach[n as usize] += reach[i as usize];
        }
    }

    reach[0]
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut adapters: Vec<u64> = content
        .trim()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    adapters.sort();

    println!("Part One: {} ", p1(adapters.clone()));
    println!("Part Two: {:?}", p2(adapters));
}
