use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Day 6");
    // Read content
    let content = fs::read_to_string("day 6/input.txt").unwrap();
    let chars: Vec<char> = content.chars().collect();
    // Part 1
    let mut letters = HashMap::new();
    letters.insert(chars[0], 1);
    *letters.entry(chars[1]).or_insert(0) += 1;
    *letters.entry(chars[2]).or_insert(0) += 1;
    *letters.entry(chars[3]).or_insert(0) += 1;
    for (i, &letter) in chars[4..].iter().enumerate() {
        letters.retain(|_, v| *v > 0);
        if letters.keys().len() == 4 {
            println!("Index of first non-repeated set of 4 letters: {}", i + 4);
            println!("{:#?}", letters);
            break
        }
        if letter.is_alphabetic() {
            *letters.entry(letter).or_insert(0) += 1;
            *letters.entry(chars[i]).or_insert(0) -= 1;
        }
    }
    // Part 2
    // let chars: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
    letters = chars[..14].iter().fold(HashMap::new(), |mut acc, &letter| {
        *acc.entry(letter).or_insert(0) += 1;
        acc
    });
    for (i, &letter) in chars[14..].iter().enumerate() {
        letters.retain(|_, v| *v > 0);
        if letters.keys().len() == 14 {
            println!("Index of first non-repeated set of 14 letters: {}", i + 14);
            println!("{:#?}", letters);
            break
        }
        if letter.is_alphabetic() {
            *letters.entry(letter).or_insert(0) += 1;
            *letters.entry(chars[i]).or_insert(0) -= 1;
        }
    }
}