use std::fs;

#[derive(Debug)]
struct Command {
    count: u32,
    from: u32,
    to: u32,
}

impl Command {
    fn apply(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.count {
            let item = stacks[self.from as usize].pop().unwrap();
            stacks[self.to as usize].push(item);
        }
    }

    fn apply_better(&self, stacks: &mut [Vec<char>]) {
        let mut items = stacks[self.from as usize].drain((stacks[self.from as usize].len() - self.count as usize)..).collect::<Vec<char>>();
        stacks[self.to as usize].append(& mut items);

        // let mut items = Vec::new();
        // for _ in 0..self.count {
        //     items.push(stacks[self.from as usize].pop().unwrap())
        // }
        // items.reverse();
        // for &item in items.iter() {
        //     stacks[self.to as usize].push(item);
        // }
    }
}


fn main() {
    println!("Day 5");
    // Read content
    let content = fs::read_to_string("day 5/input.txt").unwrap();
    // Preprocess stack (Copied from input)
    let mut stacks = vec![
        vec!['N', 'W', 'F', 'R', 'Z', 'S', 'M', 'D'],
        vec!['S', 'G', 'Q', 'P', 'W'],
        vec!['C', 'J', 'N', 'F', 'Q', 'V', 'R', 'W'],
        vec!['L', 'D', 'G', 'C', 'P', 'Z', 'F'],
        vec!['S', 'P', 'T'],
        vec!['L', 'R', 'W', 'F', 'D', 'H'],
        vec!['C', 'D', 'N', 'Z'],
        vec!['Q', 'J', 'S', 'V', 'F', 'R', 'N', 'W'],
        vec!['V', 'W', 'Z', 'G', 'S', 'M', 'R'],
    ];
    // Reverse stacks
    stacks.iter_mut().for_each(|stack| stack.reverse());
    println!("{:?}", stacks);
    // Process content
    let mut lines = content.lines();
    // Skip to first command
    while !lines.next().unwrap().is_empty() {}
    let mut moves = Vec::new();
    // Process commands
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(' ');
        parts.next(); // Skip "move"
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        parts.next(); // Skip "from"
        let from = parts.next().unwrap().parse::<u32>().unwrap();
        parts.next(); // Skip "to"
        let to = parts.next().unwrap().parse::<u32>().unwrap();
        moves.push(Command { count, from: from - 1, to: to - 1});
    }
    // println!("{:#?}", moves);
    // Part 1
    let mut stacks_part_1 = stacks.clone();
    // Apply moves
    for m in moves.iter() {
        m.apply(&mut stacks_part_1[..]);
    }
    for stack in stacks_part_1.iter() {
        print!("{:}", stack.last().unwrap());
    }
    println!();
    // Part 2
    let mut stacks_part_2 = stacks.clone();
    for m in moves.iter() {
        m.apply_better(&mut stacks_part_2[..]);
    }
    for stack in stacks_part_2.iter() {
        print!("{:}", stack.last().unwrap());
    }
    println!();

}