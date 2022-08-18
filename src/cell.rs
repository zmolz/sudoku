use std::fmt;

///////////////////////// CELL /////////////////////////////

/**
* each cell has a position described by a Coordinate struct,
  a value described by a CellVal enumuration, and a list of 
  remaining values to use in the backtracking algorithm
*/
pub struct Cell {
    val: CellVal,
    pos: Coord,
    remaining: Vec<CellVal>,
}

impl Cell {
    pub fn new(val: CellVal, pos: Coord, remaining: Vec<CellVal>) -> Cell {
        Cell {
            val,
            pos,
            remaining,
        }
    }

    pub fn pos(&self) -> &Coord {
        &self.pos
    }

    pub fn val(&self) -> CellVal {
        self.val
    }

    pub fn remaining(&self) -> &Vec<CellVal> {
        &self.remaining
    } 
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub static CELL_VALS: [CellVal; 9] = [
    CellVal::One,
    CellVal::Two,
    CellVal::Three,
    CellVal::Four,
    CellVal::Five,
    CellVal::Six,
    CellVal::Seven,
    CellVal::Eight,
    CellVal::Nine,
];

//////////////////////// CELL VAL ///////////////////////////

/**
 * CellVal enum describes the value (1-9 or None) of a Cell
 */
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CellVal {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl CellVal {
    // enums live on stack no need to borrow self or clone
    fn val(self) -> String {
        match self {
            CellVal::One => String::from("1"),
            CellVal::Two => String::from("2"),
            CellVal::Three => String::from("3"),
            CellVal::Four => String::from("4"),
            CellVal::Five => String::from("5"),
            CellVal::Six => String::from("6"),
            CellVal::Seven => String::from("7"),
            CellVal::Eight => String::from("8"),
            CellVal::Nine => String::from("9"),
        }
    }
}

impl fmt::Display for CellVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val())
    }
}

////////////////////////// COORD ///////////////////////////

/**
 * Coordinate struct defines a row/col position in the board
 */
pub struct Coord {
    i: usize,
    j: usize,
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Coord {
        // usize can not be negative
        if i > 9 || j > 9 {
            panic!("Coord values passed are not within the board's boundaries")
        }
        Coord { i, j }
    }

    pub fn row(&self) -> usize {
        self.i
    }

    pub fn col(&self) -> usize {
        self.j
    }

    // 1-9
    pub fn grid(&self) -> usize {
        // cells in first-third row are grid 1-3
        let row = if self.i <= 3 {
            1
        } else if self.i <= 6 {
            4
        } else {
            7
        };

        if self.j <= 3 {
            row
        } else if self.j <= 6 {
            row + 1
        } else {
            row + 2
        }
    }
}
