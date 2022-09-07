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
    static ref COORD_MATCH: Regex = Regex::new(r"^(\d), (\d)$").unwrap();
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

    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut val: usize = 0;

    'solved: loop {
        println!("{}", solver);

        'coord: loop {
            input.clear();
            println!("enter a coordinate to fill (row, col): ");

            io::stdin()
                .read_line(&mut input)
                .expect("error reading line");

            input = input.trim().to_string();

            if let Some(cap) = COORD_MATCH.captures(&input) {
                row = cap[1].parse().unwrap();
                col = cap[2].parse().unwrap();

                if row > 9 || col > 9 || row < 1 || col < 1 {
                    println!("error: row and col must be between 1 and 9");
                    continue 'coord;
                } else {
                    break 'coord;
                }
            }
        }

        'val: loop {
            input.clear();
            println!("enter a value to fill in: 1-9, or 0 to clear");

            io::stdin()
                .read_line(&mut input)
                .expect("error reading line");

            input = input.trim().to_string();

            let inter = input.parse::<usize>();

            match inter {
                Ok(v) => {
                    if v > 9 {
                        println!("error: value must be between 0 and 9");
                        continue 'val;
                    } else {
                        val = v;
                        break 'val;
                    }
                }
                Err(_) => {
                    println!("error: input must be a number between 0 and 9");
                    continue 'val;
                }
            }
        }

        if !solver.fill_cell(row, col, val) {
            continue 'solved;
        }

        if solver.is_solved() {
            println!("solved!");
            break 'solved;
        }
    }
}
