use std::collections::HashSet;

fn load_deck(file: &str) -> Vec<usize> {
    std::fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn score(players: Vec<Vec<usize>>) -> usize {
    let winner = (players[1].len() > 0) as usize;
    players[winner]
        .iter()
        .rev()
        .enumerate()
        .fold(0usize, |acc, val| acc + (val.0 + 1) * val.1)
}

fn part_one() {
    let mut players = vec![load_deck("player1.txt"), load_deck("player2.txt")];

    while players.iter().all(|p| p.len() > 0) {
        let p1 = players[0].remove(0);
        let p2 = players[1].remove(0);

        let winner = (p2 > p1) as usize; // false=0 if p1 is higher, 1 otherwise
        players[winner].push(p1.max(p2));
        players[winner].push(p1.min(p2));
    }

    println!("Part One: {}", score(players));
}

fn recursive_game(players: Vec<Vec<usize>>) -> (usize, Vec<Vec<usize>>) {
    let mut recursion_detector = HashSet::new();

    let mut players = players;

    while players.iter().all(|p| p.len() > 0) {
        if !recursion_detector.insert(format!("{:?}", players)) {
            return (0, players);
        }
        let p1 = players[0].remove(0);
        let p2 = players[1].remove(0);

        if players[0].len() >= p1 && players[1].len() >= p2 {
            let mut copy = players.clone();
            copy[0].truncate(p1);
            copy[1].truncate(p2);
            let (winner, _) = recursive_game(copy);
            if winner == 0 {
                players[0].push(p1);
                players[0].push(p2);
            } else {
                players[1].push(p2);
                players[1].push(p1);
            }
        } else {
            let winner = (p2 > p1) as usize; // false=0 if p1 is higher, 1 otherwise
            players[winner].push(p1.max(p2));
            players[winner].push(p1.min(p2));
        }
    }

    let winner = (players[1].len() > 0) as usize;
    (winner, players)
}

fn part_two() {
    let players = vec![load_deck("player1.txt"), load_deck("player2.txt")];
    let winner = recursive_game(players);

    println!("{:?}", score(winner.1));
}

fn main() {
    part_one();

    part_two();
}
