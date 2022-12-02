use std::fs;


fn main() {
    println!("Day 1 code");
    let contents = fs::read_to_string("day 1/input.txt")
        .expect("Failed to read string.");
    let contents_list: Vec<&str> = contents.split('\n').collect();
    let mut calories = 0;
    let mut totals = Vec::new();
    for line in contents_list {
        if line.is_empty() {
            totals.push(calories);
            calories = 0;
            continue;
        }
        let calorie = line.parse::<u32>().expect("Line was not an integer");
        calories += calorie;
    }
    totals.sort();
    println!("Max calories count: {}", totals[totals.len() - 1]);
    let top_3_sum: u32 = (totals[totals.len() - 3..])
            .iter()
            .sum();
    println!("Total of top 3: {}", top_3_sum);
}
