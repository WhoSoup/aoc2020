use std::collections::HashSet;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let p1: u32 = content
        .split("\n\n")
        .map(
            // a group
            |x| {
                x.split("\n")
                    // a declaration
                    .fold(HashSet::new(), |mut acc, answer| {
                        acc.extend(answer.chars());
                        acc
                    })
                    .len() as u32
            },
        )
        .sum();

    println!("Part One: {}", p1);

    let p2: u32 = content
        .split("\n\n")
        .map(
            // a group
            |x| {
                let decls = x.trim().split("\n");
                let total = decls.clone().count() as u32;
                decls
                    .fold(vec![0; 26], |mut acc, answer| {
                        for c in answer.chars() {
                            acc[(c as i8 - 97) as usize] += 1;
                        }
                        acc
                    })
                    .iter()
                    .filter(|x| **x == total)
                    .count()
            } as u32,
        )
        .sum();

    println!("Part Two: {}", p2);
}
