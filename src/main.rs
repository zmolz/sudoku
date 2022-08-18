mod board;
use board::Board;
use std::io;

const IMPOSSIBLE: [&str; 2] = ["IMPOSSIBLE", "I"];

const HARD: [&str; 2] = ["HARD", "H"];

const NORMAL: [&str; 2] = ["NORMAL", "N"];

const EASY: [&str; 2] = ["EASY", "E"];

const DIFFICULTY_IMPOSSIBLE: usize = 64;
const DIFFICULTY_HARD: usize = 48;
const DIFFICULTY_NORMAL: usize = 32;
const DIFFICULTY_EASY: usize = 16;

fn main() {
    let difficulties = "
    [IMPOSSIBLE, I]
    [HARD, H],
    [NORMAL, N],
    [EASY, E]";

    println!(
        "enter difficulty setting: {}", difficulties
    );

    let mut input = String::new();

    let b: Board;

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
                "error reading input, please enter a difficulty setting: {}", difficulties
            );
            input.clear();
            continue;
        }

        break;
    }

    println!("{}", b);
}
