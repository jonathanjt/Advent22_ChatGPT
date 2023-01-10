use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.tx").expect("File not found");
    let mut strategy = String::new();
    file.read_to_string(&mut strategy).expect("Failed to read file");
    let mut total_score = 0;

    let inputs: Vec<&str> = strategy.split_whitespace().collect();
    for i in (0..inputs.len()).step_by(2) {
        let opponent_choice = inputs[i].chars().next().unwrap();
        let my_choice = inputs[i+1].chars().next().unwrap();

        let (my_score, outcome) = match opponent_choice {
            'A' => {
                if my_choice == 'X' {
                    (1, 0) // loss
                } else if my_choice == 'Y' {
                    (2, 6) // win
                } else {
                    (3, 3) // draw
                }
            }
            'B' => {
                if my_choice == 'X' {
                    (1, 6) // win
                } else if my_choice == 'Y' {
                    (2, 0) // loss
                } else {
                    (3, 3) // draw
                }
            }
            'C' => {
                if my_choice == 'X' {
                    (1, 3) // draw
                } else if my_choice == 'Y' {
                    (2, 3) // draw
                } else {
                    (3, 0) // loss
                }
            }
            _ => panic!("Invalid input"),
        };

        total_score += my_score + outcome;
    }

    println!("Total score: {}", total_score);
}
