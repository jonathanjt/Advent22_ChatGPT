use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    // Open the strategy file
    let file = File::open("input.tx").unwrap();
    let reader = BufReader::new(file);

    let shape_scores = vec![(1, 'A'), (2, 'B'), (3, 'C'),(1, 'X'), (2, 'Y'), (3, 'Z')];
    let outcome_scores = vec![(0, 'A'), (3, 'B'), (6, 'C')];
    let mut shape_map = HashMap::new();
    let mut outcome_map = HashMap::new();

    for (i, shape) in shape_scores.iter().enumerate() {
        shape_map.insert(shape.1, i);
    }

    for (i, outcome) in outcome_scores.iter().enumerate() {
        outcome_map.insert(outcome.1, i);
    }

    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<&str> = line.split_whitespace().collect();
        let opponent_shape = shape_map[&chars[0].chars().next().unwrap()];
        let player_shape = shape_map[&chars[1].chars().next().unwrap()];
        let outcome = match (opponent_shape, player_shape) {
            (0, 4) => 'C',
            (1, 5) => 'C',
            (2, 3) => 'C',
            (_, _) if opponent_shape == player_shape => 'B',
            (_, _) => 'A',
        };
        let score = shape_scores[opponent_shape].0 + outcome_scores[outcome_map[&outcome]].0;
        total_score += score;
    }

    println!("Total score: {}", total_score);
}
