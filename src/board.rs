#[path = "cell.rs"]
mod cell;
use cell::{Cell, CellVal, Coord, CELL_VALS};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

const MAX_ROWS: usize = 9;
const MAX_COLS: usize = 9;

pub struct Board {
    /* we do not need a 2d matrix as the board,
    as each Cell struct contains info on
    its own position within the board.
    we need a Queue to support our
    recursive backtracking algorithm */
    cells: VecDeque<Cell>,
}

impl Board {
    pub fn new(k: usize) -> Board {
        // initialize cell Queue
        let mut b: Board = Board {
            cells: VecDeque::new(),
        };

        // start recursive algorithm with 1st row 1st col
        b.fill_cells(1, 1, None);

        // remove values from filled-in board
        b.remove_k_cells(k);

        b
    }

    fn fill_cells(&mut self, i: usize, j: usize, remaining: Option<Vec<CellVal>>) -> () {
        // base case 1: board is filled in and we reached the 10th row
        if i > MAX_ROWS {
            return;
        }

        let pos: Coord = Coord::new(i, j);

        let mut options: Vec<CellVal>;
        // determine which CellVal options to use
        if let Some(rem) = remaining {
            // if this is intended to be a backtracking call, use remaining cells
            options = rem
        } else {
            // iterate over cells in board and take those in the same col, row, or grid
            let neighbors: HashSet<CellVal> = self.get_neighbors(&pos);

            // find set difference between cell val options and the neighbors
            options = cell_vals_diff(neighbors);
        }

        // base-case 2, no valid options, could be entered by backtracking call or a next cell call
        if options.is_empty() {
            /*
               we will have "psuedo recursive-backtracking", where
               the backtracking is not neccesarily going to pop a frame off the
               stack as you might expect, but rather we will control backtracking by
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
               Which, ill say, is not idiomatically the way
               to recurse but nevertheless will be easier to code, and still
               θ(n^2) in terms of time complexity.
            */

            // remove the last cell from the queue
            let last = self.cells.pop_back().unwrap(); // safe to unwrap

            let rem = last.remaining();

            // recursive step backwards
            let pos = last.pos();
            let i = pos.row();
            let j = pos.col();

            self.fill_cells(i, j, Some(rem.to_owned()));
        } else {
            // shuffle array if we have options (would reshuffle on a backtracking call, change later)
            let mut rng = thread_rng();
            options.shuffle(&mut rng);

            // get value to add
            let val = options[0];
            let remaining = options[1..].to_vec();

            // add value
            self.cells.push_back(Cell::new(val, pos, remaining));

            // recursive step forwards
            let (i, j) = if j == MAX_COLS { (i + 1, 1) } else { (i, j + 1) };

            self.fill_cells(i, j, None)
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

    fn remove_k_cells(&mut self, k: usize) -> () {
        let mut cell_indices: Vec<usize> = (0..81).collect();
        let mut rng = thread_rng();
        cell_indices.shuffle(&mut rng);
        let indices_to_remove = cell_indices[..k].to_vec();

        for i in indices_to_remove {
            self.cells[i].to_empty_cell();
        }
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

        write!(f, "{}", string_builder)
    }
}

//////////////////////// SOLVER ///////////////////////////

pub struct Solver {
    board: Board,
    is_solved: bool,
    cells_left: usize,
}

impl Solver {
    pub fn new(board: Board) -> Solver {


        Solver { board: board, is_solved: false, cells_left: 10 }
    }

    pub fn auto_solve(&mut self) -> () {
        
    }

    pub fn check_cells_left(&self) -> usize {

    }
}