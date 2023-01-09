use std::io;
use std::iter::FromIterator;

fn main() {
    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input into a Vec of Vecs of i32s
    let mut calories: Vec<Vec<i32>> = Vec::new();
    let mut current_elf_calories = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            // End of input for current elf
            calories.push(current_elf_calories);
            current_elf_calories = Vec::new();
        } else {
            // Parse the calorie count for the current elf
            let calorie_count = line.parse().unwrap();
            current_elf_calories.push(calorie_count);
        }
    }
    if !current_elf_calories.is_empty() {
        // Add the remaining calories for the last elf
        calories.push(current_elf_calories);
    }

    // Calculate the total calories for each elf
    let elf_calorie_totals: Vec<i32> = calories
        .iter()
        .map(|calories| calories.iter().sum())
        .collect();

    // Find the elf with the most calories
    let max_calories = elf_calorie_totals.iter().max().unwrap();

    // Print the total calories for the elf with the most calories
    println!("{}", max_calories);
}
