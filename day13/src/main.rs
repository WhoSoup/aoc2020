fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();
    let time = lines.next().unwrap().parse::<u64>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| match x.parse::<u64>() {
            Ok(y) => y,
            Err(_) => 0,
        });
    {
        // part one
        let p1 = &busses
            .clone()
            .filter(|&y| y > 0)
            .map(|y| (y, y - (time % y)))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        println!("{:?}", p1.0 * p1.1);
    }
}
