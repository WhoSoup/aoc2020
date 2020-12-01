use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("bad file");
    let mut v: Vec<u64> = Vec::new();
    for line in content.lines() {
        let num: u64 = line.parse().unwrap();
        v.push(num);
    }

    let max = v.len()-1;

    for i in 0..max {
        for j in i+1..max {
            if v[i] + v[j] == 2020 {
                println!("Part One:");
                println!("{} + {} = {}", v[i], v[j], v[i] + v[j]);
                println!("{} * {} = {}", v[i], v[j], v[i] * v[j]);
            }
        }
    }

    
    for i in 0..max {
        for j in i+1..max {
            for k in j+1..max {
                if v[i] + v[j] + v[k] == 2020 {
                    println!("Part Two:");
                    println!("{} + {} + {} = {}", v[i], v[j], v[k], v[i] + v[j] + v[k]);
                    println!("{} * {} * {} = {}", v[i], v[j], v[k], v[i] * v[j] * v[k]);
                }
            }
        }
    }
    
}
