#[path = "cell.rs"]
mod cell;
use cell::{Cell, CellVal, Coord, CELL_VALS, Error};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

const MAX_ROWS: usize = 9;
const MAX_COLS: usize = 9;

pub struct Board {
    /* we do not need a 2d matrix as the board,
    as each Cell struct contains info on
    its own position within the board.
    we need a Queue to support our
    recursive backtracking algorithm */
    cells: VecDeque<Cell>,

    /* this attribute will help us implement
    an "is_solved" function by mapping each coord
    in the solved board to a value.  */
    pos_to_val: HashMap<Rc<Coord>, CellVal>,
}

impl Board {
    pub fn new(k: usize) -> Board {
        // initialize cell Queue
        let mut b: Board = Board {
            cells: VecDeque::new(),
            pos_to_val: HashMap::new(),
        };

        // start recursive algorithm with 1st row 1st col
        b.fill_cells(1, 1, None);

        // fill in pos_to_cell
        b.map_coord_to_cells();

        // remove values from filled-in board
        b.remove_k_cells(k);

        // return ready-to-solve board
        b
    }

    fn fill_cells(&mut self, i: usize, j: usize, remaining: Option<Vec<CellVal>>) {
        // base case 1: board is filled in and we reached the 10th row
        if i > MAX_ROWS {
            return;
        }

        let pos: Coord = Coord::new(i, j).unwrap(); // safe, programmer controlled

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
               so theres no logic neccesary to evaluate what that coord will be.

               Downside is obviously that the algorithm will be less
               memory efficient in terms of concurrent stack size,
               but the upside is that the amount of total frames added will not
               change, and we do not need to worry about having recursive calls
               return values to the caller, but rather pass values to the next call.
               Which, ill admit, is not idiomatically the way
               to recurse but nevertheless will be easier to code, and still
               θ(n^2) in terms of time complexity. (with n = num rows | col)
            */

            // remove the last cell from the queue
            let last = self.cells.pop_back().unwrap(); // safe to unwrap

            let remaining = Some(last.remaining().to_owned());

            // recursive step backwards
            let pos = last.pos();
            let i = pos.row();
            let j = pos.col();

            self.fill_cells(i, j, remaining);
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
            let (i, j) = if j == MAX_COLS {
                (i + 1, 1)
            } else {
                (i, j + 1)
            };

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

    fn map_coord_to_cells(&mut self) {
        for cell in &self.cells {
            let pos = Rc::new(*cell.pos());
            let cellval = cell.val();
            self.pos_to_val.insert(pos, cellval);
        }
    }

    fn remove_k_cells(&mut self, k: usize) {
        let mut cell_indices: Vec<usize> = (0..81).collect();
        let mut rng = thread_rng();
        cell_indices.shuffle(&mut rng);
        let indices_to_remove = cell_indices[..k].to_vec();

        for i in indices_to_remove {
            self.cells[i].to_empty_cell();
        }
    }

    pub fn is_solved(&self) -> bool {

        for cell in &self.cells {
            let pos = cell.pos();
            let val = cell.val();

            if self.pos_to_val[pos] != val {
                return false;
            }
        }

        true
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

// takes ownership of the values in the queue and converts them to a HashMap
fn queue_to_hashmap(queue: VecDeque<Cell>) -> HashMap<Rc<Coord>, CellVal> {
    let mut ret: HashMap<Rc<Coord>, CellVal> = HashMap::new();

    for cell in queue {
        let pos = Rc::new(*cell.pos());
        let cellval = cell.val();
        ret.insert(pos, cellval);
    }

    ret
}

#[derive(Debug)]
pub struct Solver {
    // solved board
    solved: HashMap<Rc<Coord>, CellVal>,

    // the board to solve
    active: HashMap<Rc<Coord>, CellVal>,

    // cells given as clues
    clues: HashSet<Rc<Coord>>,
}

impl Solver {
    pub fn new(board: Board) -> Solver {
        let mut clues: HashSet<Rc<Coord>> = HashSet::new();
        for cell in &board.cells {
            let pos = Rc::new(*cell.pos());
            clues.insert(pos);
        }
        Solver {
            solved: board.pos_to_cell,
            active: queue_to_hashmap(board.cells),
            clues: clues,
        }
            /* we want to take ownership here
            and convert the queue to a hashmap
            so that interaction with the active board will be easier
            and we can drop the Cell struct which deals with
            the remaining values, which is not necessary for the solver.
            comparing two of the same data structure is easier than
            comparing two different ones */
    }

    /* very simple is_solved function compared to potentially having
    to iterate over the cells queue, deconstruct their position and value,
    etc */
    pub fn is_solved(&self) -> bool {
        self.solved == self.active
    }

    pub fn fill_cell(&mut self, row: usize, col: usize, val: usize) -> Result<(), Error>{
        let pos = Coord::new(row, col);
        let coord: Rc<Coord>;
        match pos {
            Ok(c) => (coord = Rc::new(c)),
            Err(e) => return Err(e),
        }

        if self.clues.contains(&coord) {
            return Err(Error::overwrite_error());
        }
        
        let val: CellVal = CellVal::new(val);
        self.active.insert(coord, val);
        Ok(())
    }

}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        ret.push_str("\n-------------------------\n");

        for i in 1..=MAX_ROWS {
            ret.push_str("| ");
            for j in 1..=MAX_COLS {
                let pos = Coord::new(i, j).unwrap();
                let val = self.active.get(&pos).unwrap();

                ret.push_str(&format!("{} ", val));
                if j % 3 == 0 {
                    ret.push_str("| ");
                }
            }

            if i % 3 == 0 {
                ret.push_str("\n-------------------------\n");
            } else {
                ret.push_str("\n")
            }
        }

        write!(f, "{}", ret)
    }
}