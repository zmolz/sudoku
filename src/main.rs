mod board;
use board::{Board, Solver};
use lazy_static::lazy_static;
use regex::Regex;
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

lazy_static! {
    static ref COORD_MATCH: Regex = Regex::new(r"^(\d)(, |,| )(\d)(, |,| )(\d)$").unwrap();
}

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
            b = Board::new(DIFFICULTY_EASY);
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

    let mut solver = Solver::new(b);

    let mut row: usize;
    let mut col: usize;
    let mut val: usize;

    loop {
        println!("{}", solver);

        input.clear();
        println!("enter a cell and val (1-9, 0 to clear) as following:\trow, col, val\n\t");
        io::stdin()
            .read_line(&mut input)
            .expect("error reading line");
        input = input.trim().to_string();
        if let Some(cap) = COORD_MATCH.captures(&input) {
            row = cap[1].parse().unwrap();
            col = cap[3].parse().unwrap();
            val = cap[5].parse().unwrap(); // safe to unwrap, validated by regex
        } else {
            println!("error reading input, please try again, following format");
            continue; // invalid input, try again
        }

        match solver.fill_cell(row, col, val) {
            Ok(_) => (),
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        }
        if solver.is_solved() {
            println!("solved!");
            break;
        }
    }
}
