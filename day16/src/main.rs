#[derive(Debug, Clone, Copy)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Range {
        Range { start, end }
    }

    fn inside(&self, n: u32) -> bool {
        n >= self.start && n <= self.end
    }
}
#[derive(Debug, Clone)]
struct Rule {
    name: String,
    ranges: Vec<Range>,
}

impl Rule {
    fn new(name: String) -> Rule {
        Rule {
            ranges: vec![],
            name,
        }
    }

    fn parse(s: &str) -> Rule {
        let mut fsplit = s.split(": ");
        let mut r = Rule::new(fsplit.next().unwrap().trim().to_string());

        for mut n in fsplit
            .next()
            .unwrap()
            .split(" or ")
            .map(|x| x.split("-").map(|y| y.parse::<u32>().unwrap()))
        {
            r.add_range(n.next().unwrap(), n.next().unwrap())
        }

        r
    }

    fn add_range(&mut self, start: u32, end: u32) {
        self.ranges.push(Range::new(start, end))
    }

    fn valid(&self, n: u32) -> bool {
        self.ranges.iter().any(|x| x.inside(n))
    }
}

fn main() {
    let c_rules = std::fs::read_to_string("input-rules.txt").unwrap();
    let c_nearby = std::fs::read_to_string("input-nearby_tickets.txt").unwrap();

    let rules: Vec<Rule> = c_rules.lines().map(|line| Rule::parse(line)).collect();

    let nearby: Vec<Vec<u32>> = c_nearby
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    let mut nearby_valid: Vec<Vec<u32>> = vec![];
    let mut p1 = 0;
    for ticket in &nearby {
        for num in ticket {
            if !rules.iter().any(|rule| rule.valid(*num)) {
                p1 += num;
            } else {
                nearby_valid.push(ticket.clone());
            }
        }
    }

    println!("Part One: {}", p1);
}
