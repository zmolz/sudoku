mod board;
use board::{Board};

use std::io;

const IMPOSSIBLE: [&str; 2] = ["IMPOSSIBLE", "I"];

const HARD: [&str; 2] = ["HARD", "H"];

const NORMAL: [&str; 2] = ["NORMAL", "N"];

const EASY: [&str; 2] = ["EASY", "E"];

// all of these sizes guarantee 1 unique solution
const DIFFICULTY_IMPOSSIBLE: usize = 64;
const DIFFICULTY_HARD: usize = 48;
const DIFFICULTY_NORMAL: usize = 32;
const DIFFICULTY_EASY: usize = 16;

const DIFFICULTIES: &str = "
[IMPOSSIBLE/I]
[HARD/H],
[NORMAL/N],
[EASY/E]";

fn main() {
    println!("enter difficulty setting: {}", DIFFICULTIES);

    let mut input = String::new();

    let b: Board;

    // set game difficulty (difficulty here is measured by amount of clues)
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("error reading line");

        input = input.trim().to_uppercase();

        if IMPOSSIBLE.contains(&&input[..]) {
            b = Board::new(DIFFICULTY_IMPOSSIBLE);
        } else if HARD.contains(&&input[..]) {
            b = Board::new(DIFFICULTY_HARD);
        } else if NORMAL.contains(&&input[..]) {
            b = Board::new(DIFFICULTY_NORMAL);
        } else if EASY.contains(&&input[..]) {
            b = Board::new(DIFFICULTY_EASY)
        } else {
            println!(
                "error reading input, please enter a difficulty setting: {}",
                DIFFICULTIES
            );
            input.clear();
            continue;
        }

        break;
    }

    println!("{}", b);

    
}
