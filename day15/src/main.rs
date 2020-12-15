use std::collections::HashMap;

struct Game {
    spoken: HashMap<i32, usize>,
    next: i32,
    turn: usize,
}

impl Game {
    fn new(init: Vec<i32>) -> Game {
        let mut g = Game {
            spoken: HashMap::new(),
            next: 0,
            turn: 0,
        };
        for n in init {
            g.spoken.insert(n, g.turn);
            g.turn += 1;
        }
        println!("{:?}", g.spoken);
        g
    }

    fn speak(&mut self) {
        if let Some(when) = self.spoken.insert(self.next, self.turn) {
            self.next = (self.turn - when) as i32;
        } else {
            self.next = 0;
        }

        self.turn += 1;
    }

    fn run(&mut self, turns: usize) -> i32 {
        while self.turn < turns - 1 {
            self.speak();
        }
        self.next
    }
}

fn main() {
    let mut g = Game::new(vec![1, 12, 0, 20, 8, 16]);
    println!("Part One: {}", g.run(2020));

    g = Game::new(vec![1, 12, 0, 20, 8, 16]);
    println!("Part Two: {}", g.run(30000000));
}
