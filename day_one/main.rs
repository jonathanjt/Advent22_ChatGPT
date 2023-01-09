use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the input file
    let file = File::open("input.tx").unwrap();
    let reader = BufReader::new(file);
    let mut calories = Vec::new();
    let mut current_calories = 0;
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
    // Sort the calories in descending order
    calories.sort_by(|a, b| b.cmp(a));
    // Take the top three elements
    let top_three: Vec<i32> = calories.into_iter().take(3).collect();
    // Calculate the total calories
    let total_calories: i32 = top_three.into_iter().sum();
    // Print the result
    println!("{}", total_calories);
}
