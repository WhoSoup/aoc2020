use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let ids = content.lines()
        .map(|x| x.chars().enumerate()
            .fold(0, |acc, (pos, c)| 
                acc + if c == 'B' || c == 'R' {
                    (2 as i32).pow(9-pos as u32)
                } else {
                    0
                }
            )
        );

    println!("{}", ids.clone().max().unwrap());


    let mut i = ids.clone().min().unwrap();
    for n in ids.sorted() {
        if n != i {
            println!("Part Two: {}", i);
            break;
        }
        i += 1;
    }   
}
