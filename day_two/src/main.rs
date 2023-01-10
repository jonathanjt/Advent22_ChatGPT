use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.tx").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        let opponent = parts[0];
        let my_guess = parts[1];

        let opponent_move = match opponent {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            _ => panic!("Invalid opponent move"),
        };

        let my_move = match my_guess {
            "X" => "Rock",
            "Y" => "Paper",
            "Z" => "Scissors",
            _ => panic!("Invalid my move"),
        };

        // Check the result of the game and update the total score
        if opponent_move == my_move {
            total_score += 3;
            match my_guess {
                "X" => total_score += 1,
                "Y" => total_score += 2,
                "Z" => total_score += 3,
                _ => {}
            }
        } else if (opponent_move == "Rock" && my_move == "Scissors") || (opponent_move == "Scissors" && my_move == "Paper") || (opponent_move == "Paper" && my_move == "Rock") {
            total_score += 0;
            match my_guess {
                "X" => total_score += 1,
                "Y" => total_score += 2,
                "Z" => total_score += 3,
                _ => {}
            }
        } else {
            total_score += 6;
            match my_guess {
                "X" => total_score += 1,
                "Y" => total_score += 2,
                "Z" => total_score += 3,
                _ => {}
            }
        }
    }

    println!("Total Score: {}", total_score);
}
