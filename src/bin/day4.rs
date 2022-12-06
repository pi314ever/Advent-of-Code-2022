use std::fs;

#[derive(Debug)]
struct Assignment {
    low: u32,
    high: u32,
}

#[derive(Debug)]
struct Pair {
    first: Assignment,
    second: Assignment,
}

impl Pair {
    fn new(first: Assignment, second: Assignment) -> Pair {
        Pair { first, second }
    }

    fn is_contained(&self) -> bool {
        (self.first.low <= self.second.low && self.second.high <= self.first.high )||
        (self.second.low <= self.first.low && self.first.high <= self.second.high)
    }

    fn is_overlapped(&self) -> bool {
        (self.first.low <= self.second.low && self.second.low <= self.first.high) ||
        (self.second.low <= self.first.low && self.first.low <= self.second.high)
    }
}

fn main() {
    println!("Day 4");
    // Read input
    let contents = fs::read_to_string("day 4/input.txt").unwrap();
    // Parse input
    let mut pairs: Vec<Pair> = Vec::new();
    for line in contents.lines() {
        let mut assignments_split = line.split(',');
        let first = assignments_split.next().unwrap();
        let second = assignments_split.next().unwrap();
        let mut first_split = first.split('-');
        let mut second_split = second.split('-');
        let first_assignment = Assignment {
            low: first_split.next().unwrap().parse::<u32>().unwrap(),
            high: first_split.next().unwrap().parse::<u32>().unwrap(),
        };
        let second_assignment = Assignment {
            low: second_split.next().unwrap().parse::<u32>().unwrap(),
            high: second_split.next().unwrap().parse::<u32>().unwrap(),
        };
        pairs.push(Pair::new(first_assignment, second_assignment));
    }
    println!("Line count: {}", contents.lines().count());

    // Part 1
    let mut count = pairs.iter().clone().filter(|&pair| {
        pair.is_contained()
    }).count();
    println!("Part 1: {}", count);

    // Part 2
    count = pairs.iter().filter(|&pair| {
        pair.is_overlapped()
    }).count();
    println!("Part 2: {}", count);

}