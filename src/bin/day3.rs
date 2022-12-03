use std::fs;
use std::collections::{HashMap, HashSet};

fn part_1(contents_list: &[&str], values: &HashMap<char, i32>) {
    let mut total = 0;
    for &line in contents_list {
        let rucksack: Vec<char> = line.chars().collect();
        // Separate into compartments
        let first_compartment: HashSet<char> = HashSet::from_iter(rucksack[..rucksack.len() / 2].iter().cloned());
        let second_compartment: HashSet<char> = HashSet::from_iter(rucksack[rucksack.len() / 2..].iter().cloned());
        // Find intersection
        let items = first_compartment.intersection(&second_compartment);
        let mut num_items = 0;
        // Iterate through all items (should only contain 1)
        for item in items {
            total += values.get(item).expect(&("Found invalid item ".to_string() + &String::from(*item)));
            num_items += 1;
        }
        if num_items != 1 {
            println!("WARNING: {} items intersection", num_items);
            println!("{:#?} {:#?}", first_compartment, second_compartment);
        }
    }
    println!("Total value of items: {}", total);
}

fn part_2(contents_list: &[&str], values: &HashMap<char, i32>) {
    let mut total = 0;
    let mut elves: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    for (i, &line) in contents_list.iter().enumerate() {
        // Rolling overwrite for groups
        elves[i % 3] = HashSet::from_iter(line.chars().collect::<Vec<char>>().iter().cloned());
        if i % 3 == 2 {
            // Calculate elf group item every 3 lines
            let mut iter = elves.iter();
            // Initialize items to first elf
            let mut items = iter.next().unwrap().clone();
            // Sequentially intersect with other elves
            for elf in iter {
                items = items.intersection(elf).cloned().collect();
            }
            let mut num_items = 0;
            for item in items {
                total += values.get(&item).expect(&("Found invalid item ".to_string() + &String::from(item)));
                num_items += 1;
            }
            if num_items != 1 {
                println!("WARNING: {} items intersection", num_items);
            }
        }
    }
    println!("Total value of items: {}", total);
}

fn main() {
    println!("Day 3");
    // Read file
    let contents = fs::read_to_string("day 3/input.txt")
    .expect("Unable to read file");
    let contents_list: Vec<&str> = contents.split('\n').collect();

    let values: HashMap<char, i32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    println!("Part 1:");
    part_1(&contents_list, &values);

    println!("Part 2:");
    part_2(&contents_list, &values);
}