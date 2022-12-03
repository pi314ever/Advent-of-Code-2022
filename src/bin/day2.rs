use std::fs;
use std::collections::HashMap;

fn calculate_points(contents_list: &[&str], point_mapping: HashMap<&str, i32>) -> i32 {
    let mut points = 0;
    for &line in contents_list {
        if line.is_empty() {
            continue
        }
        points += point_mapping.get(line).expect("Mapping not found");
    }
    points
}

fn part_1(contents_list: &[&str]) {
    // A, X: Rock     +1
    // B, Y: Paper    +2
    // C, Z: Scissors +3
    // Win: +6, Draw: +3, Lose: +0
    //
    let point_mapping = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6)
    ]);

    println!("Total points: {}", calculate_points(contents_list, point_mapping));
}

fn part_2(contents_list: &[&str]) {
    // X: Lose
    // Y: Draw
    // Z: Win
    let point_mapping = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7)
    ]);

    println!("Total points: {}", calculate_points(contents_list, point_mapping))
}

fn main() {
    println!("Day 2 Code");
    // Read file
    let contents = fs::read_to_string("day 2/input.txt")
    .expect("Unable to read file");
    let contents_list: Vec<&str> = contents.split('\n').collect();

    println!("Part 1:");
    part_1(&contents_list);

    println!("Part 2:");
    part_2(&contents_list);
}