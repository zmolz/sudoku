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
        b.fill_cells(1, 1);

        // return filled-in board
        b
    }

    fn fill_cells(&mut self, i: usize, j: usize) -> () {
        // base case 1: board is filled in and we reached the 10th row
        if i == 10 {
            return;
        }

        let pos: Coord = Coord::new(i, j);

        // iterate over cells in board and take those in the same col, row, or grid
        let neighbors: HashSet<CellVal> = self.get_neighbors(&pos);

        // find set difference between cell val options and the neighbors
        let mut options: Vec<CellVal> = cell_vals_diff(neighbors);

        // DEBUG
        if options.is_empty() {
            println!("{}", self);
            panic!("No Valid Options Left")
        }

        // shuffle array
        let mut rng = thread_rng();
        options.shuffle(&mut rng);

        // get value to add
        let val = options[0];

        // add value
        self.cells.push_back(Cell::new(val, pos));


        // recursive step
        let (i, j) = if j == 9 {
            (i+1, 1)
        } else {
            (i, j+1)
        };

        self.fill_cells(i, j)
    }

    fn get_neighbors(&self, pos: &Coord) -> HashSet<CellVal> {
        let mut neighbors: HashSet<CellVal> = HashSet::new();

        for cell in &self.cells {
            let coord: &Coord = cell.pos();

            if coord.row() == pos.row()
                || coord.col() == pos.col()
                || coord.grid() == pos.grid()
            {
                neighbors.insert(cell.val());
            }
        }

        neighbors
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_builder = String::new();

        for i in 0..self.cells.len() {
            
            if (i+1) % 9 == 0 {
                let to_add = format!("{}\n", self.cells[i]);
                string_builder.push_str(&to_add);
            } else if (i+1) % 3 == 0 {
                let to_add = format!("{} | ", self.cells[i]);
                string_builder.push_str(&to_add);
            } else {
                let to_add = format!("{} ", self.cells[i]);
                string_builder.push_str(&to_add);
            }

            if (i+1) % 27 == 0 {
                string_builder.push_str("---------------------\n"); 
            }
        }

        write!(f, "{}", string_builder)
    }
}

fn cell_vals_diff(neighbors: HashSet<CellVal>) -> Vec<CellVal> {
    let mut ret: Vec<CellVal> = Vec::new();

    // enums live on stack, no need to borrow
    for cell_val in CellVal::vals() {
        // O(1), no iteration. efficient
        if !neighbors.contains(&cell_val) {
            ret.push(cell_val);
        }
    }

    ret
}

