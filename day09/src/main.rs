const PREAMBLE: usize = 25;

fn part_one(data: &Vec<u64>) -> u64 {
    let ok = |n: usize| {
        for i in (n - PREAMBLE)..n - 1 {
            for j in i + 1..n {
                if data[i] + data[j] == data[n] {
                    return true;
                }
            }
        }
        false
    };

    for i in PREAMBLE..data.len() {
        if !ok(i) {
            return data[i];
        }
    }
    panic!("algorithm wrong");
}

fn part_two(target: u64, data: &Vec<u64>) {
    for i in 0..data.len() {
        let mut sum = data[i];
        let mut last = i;
        let mut sub = vec![data[i]];

        while sum < target {
            last += 1;
            sum += data[last];
            sub.push(data[last]);
        }

        if sum == target && sub.len() > 1 {
            sub.sort();
            println!("Part Two: {}", sub.first().unwrap() + sub.last().unwrap())
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let data: Vec<u64> = content.trim().lines().map(|x| x.parse().unwrap()).collect();

    let p1 = part_one(&data);
    println!("Part One: {}", p1);

    part_two(p1, &data);
}
