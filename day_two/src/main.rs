fn main() {
    let strategy = "AYBXCZ"; // this would be the input in real case
    let mut total_score = 0;

    for (i, c) in strategy.chars().enumerate() {
        let opponent_choice = c;
        let my_choice = strategy.chars().nth(i + 1).unwrap();

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
