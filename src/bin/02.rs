use std::io;

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn char_to_move(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => {
            panic!();
        }
    }
}

fn move_to_value(m: Move) -> u32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn char_to_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => {
            panic!();
        }
    }
}

fn get_response(them: Move, outcome: Outcome) -> Move {
    match (outcome, them) {
        (Outcome::Win, Move::Scissors)
        | (Outcome::Lose, Move::Paper)
        | (Outcome::Draw, Move::Rock) => Move::Rock,
        (Outcome::Win, Move::Rock)
        | (Outcome::Lose, Move::Scissors)
        | (Outcome::Draw, Move::Paper) => Move::Paper,
        (Outcome::Win, Move::Paper)
        | (Outcome::Lose, Move::Rock)
        | (Outcome::Draw, Move::Scissors) => Move::Scissors,
    }
}

fn score(them: Move, me: Move) -> u32 {
    let play = move_to_value(me);
    let outcome = match (them, me) {
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => 0,
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => 3,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 6,
    };
    play + outcome
}

fn process_line(input: &String) -> (Move, Move) {
    let mut parsed = input.chars();
    (
        char_to_move(parsed.nth(0).unwrap()),
        // Apparently nth consumes items? Should clean this up...
        char_to_move(parsed.nth(1).unwrap()),
    )
}

fn process_line_pt2(input: &String) -> (Move, Move) {
    let mut parsed = input.chars();
    let them = char_to_move(parsed.nth(0).unwrap());
    (
        them,
        // Apparently nth consumes items? Should clean this up...
        get_response(them, char_to_outcome(parsed.nth(1).unwrap())),
    )
}

fn main() {
    let input: Vec<String> = io::stdin().lines().map(|val| val.unwrap()).collect();
    let moves: Vec<(Move, Move)> = input.iter().map(|val| process_line(val)).collect();
    let total_score: u32 = moves.iter().map(|val| score(val.0, val.1)).sum();
    println!("{}", total_score);
    let moves_pt2: Vec<(Move, Move)> = input.iter().map(|val| process_line_pt2(val)).collect();
    let total_score_pt2: u32 = moves_pt2.iter().map(|val| score(val.0, val.1)).sum();
    println!("{}", total_score_pt2);
}
