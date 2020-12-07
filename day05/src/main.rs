use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let ids = content.lines().map(|x| x.chars().map(|c|
        match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => 'x',
        }
    ).collect::<String>()
).map(|s| isize::from_str_radix(&s, 2).unwrap() as i32);

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
