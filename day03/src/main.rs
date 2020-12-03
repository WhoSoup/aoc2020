
fn move_xy(map: &Vec<Vec<bool>>, horizontal: usize, vertical: usize) -> i64 {
    let mut count = 0;
    let mut hpos: usize = 0;
    for row in (0..map.len()).step_by(vertical) {
        if map[row][hpos] {
            count += 1;
        }
        hpos += horizontal;
        hpos %= map[row].len();
    }
    count
}

fn main() {
    let content: Vec<Vec<bool>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x
                        .trim()
                        .chars()
                        .map(|x| x == '#')
                        .collect()
                    )
        .collect();

    println!("Part One: {}", move_xy(&content, 3, 1));
    println!("Part Two: {}", 
        move_xy(&content, 1, 1)
        * move_xy(&content, 3, 1)
        * move_xy(&content, 5, 1)
        * move_xy(&content, 7, 1)
        * move_xy(&content, 1, 2)
    );
}
