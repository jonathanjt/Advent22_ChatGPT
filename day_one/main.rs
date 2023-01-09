use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the input file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut max_calories = 0;
    let mut current_calories = 0;
    let mut calories = Vec::new();
    // Parse the input
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            let c: i32 = line.parse().unwrap();
            current_calories += c;
        }
    }
    calories.push(current_calories);
    // Find the elf carrying the most calories
    for c in calories {
        if c > max_calories {
            max_calories = c;
        }
    }
    // Print the result
    println!("{}", max_calories);
}
