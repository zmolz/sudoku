#[path = "cell.rs"]
mod cell;
use cell::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

pub struct Board {
    /* we do not need a 2d matrix as the board,
    as each Cell struct contains info on
    its own position within the board.
    we need a Queue to support our
    recursive backtracking algorithm */
    cells: VecDeque<Cell>,
}

impl Board {
    pub fn new() -> Board {
        // initialize cell Queue
        let mut b: Board = Board {
            cells: VecDeque::new(),
        };

        // start recursive algorithm with 1st row 1st col
        b.fill_cells(1, 1, vec![], false);

        // return filled-in board
        b
    }

    fn fill_cells(&mut self, i: usize, j: usize, remaining: Vec<CellVal>, flag: bool) -> () {
        println!("{}\n\n", self);
        // base case 1: board is filled in and we reached the 10th row
        if i == 10 {
            return;
        }

        let pos: Coord = Coord::new(i, j);

        let neighbors: HashSet<CellVal>;
        let mut options: Vec<CellVal>;
        if !flag {
            neighbors= self.get_neighbors(&pos);
            options = cell_vals_diff(neighbors);
        } else {
            options = remaining
        }
        // iterate over cells in board and take those in the same col, row, or grid
        

        // find set difference between cell val options and the neighbors

        // base-case 2
        if options.is_empty() {
            /*
               if options is empty, we need to try another cellval
               for the previous cell. to do this we must keep track of
               cellvals we have not used for the previous cell, which will be
               options[1..]. We need keep that vector unique to each frame.
               (i.e. no shared mutability so no smart pointers YAYYYY)
               every time that the remaining
               cellvals array is empty, we backtrack, try another value, take
               a step, if options is empty again we backtrack yet again, and if
               the cell now has 0 choice we backtrack two cells. etc.

               we will have "psuedo recursive-backtracking" as i will call it, where
               the backtracking is not neccesarily going to pop a frame off the
               stack as it should, but rather we will control backtracking by
               popping an element off the cell deque to remove the cell from the board
               and then making the next recursive call by calling fill_cells with
               the previous coord passed (which is just the coord of the last cell we just popped,
               so theres no logic or control flow neccesary to evaluate what that
               coord will be.

               Downside is obviously that the algorithm will be less
               memory efficient in terms of concurrent stack size,
               but the upside is that the amount of total frames added will not
               change, and we do not need to worry about having recursive calls
               return values to the caller, but rather pass values to the next call.
               everything will be callee side, which is not idiomatically the way
               to recurse but nevertheless will be easier to code, and still
               O(n^2) in terms of time complexity.
            */

            // we have removed the last cell from the queue
            let last = self.cells.pop_back().unwrap(); // safe to unwrap

            let rem = last.remaining();

            // recursive step backwards
            let pos = last.pos();
            let i = pos.row();
            let j = pos.col();
            self.fill_cells(i, j, rem.to_owned(), true);

        } else {
            // shuffle array if we have options
            let mut rng = thread_rng();
            options.shuffle(&mut rng);

            // get value to add
            let val = options[0];
            let remaining = options[1..].to_vec();

            // add value
            self.cells.push_back(Cell::new(val, pos, remaining));

            // recursive step forwards
            let (i, j) = if j == 9 { (i + 1, 1) } else { (i, j + 1) };

            self.fill_cells(i, j, vec![], false)
        }
    }

    fn get_neighbors(&self, pos: &Coord) -> HashSet<CellVal> {
        let mut neighbors: HashSet<CellVal> = HashSet::new();

        for cell in &self.cells {
            let coord: &Coord = cell.pos();

            if coord.row() == pos.row() || coord.col() == pos.col() || coord.grid() == pos.grid() {
                neighbors.insert(cell.val());
            }
        }

        neighbors
    }
}

// strictly for user-facing interface
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_builder = String::new();

        let line = "-------------------------\n";

        let mut to_add;

        string_builder.push_str(line);
        for i in 0..self.cells.len() {
            if (i + 1) % 9 == 1 {
                to_add = format!("| {} ", self.cells[i]);
                string_builder.push_str(&to_add);
            } else if (i + 1) % 9 == 0 {
                to_add = format!("{} |\n", self.cells[i]);
                string_builder.push_str(&to_add);
            } else if (i + 1) % 3 == 0 {
                to_add = format!("{} | ", self.cells[i]);
                string_builder.push_str(&to_add);
            } else {
                to_add = format!("{} ", self.cells[i]);
                string_builder.push_str(&to_add);
            }

            if (i + 1) % 27 == 0 {
                string_builder.push_str(line);
            }
        }
        // string_builder.push_str(line);
        write!(f, "{}", string_builder)
    }
}

fn cell_vals_diff(neighbors: HashSet<CellVal>) -> Vec<CellVal> {
    let mut ret: Vec<CellVal> = Vec::new();

    for cell_val in CELL_VALS {
        // O(1), no iteration. efficient
        if !neighbors.contains(&cell_val) {
            ret.push(cell_val);
        }
    }

    ret
}
