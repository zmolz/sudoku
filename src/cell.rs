use std::fmt;
#[derive(Debug, Clone)]

pub struct Error {
    message: String,
}

impl Error {
    fn bounds_error() -> Error {
        Error {
            message: "Index out of bounds".to_string(),
        }
    }

    pub fn overwrite_error() -> Error {
        Error {
            message: "Cannot overwrite cell given as clue".to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


///////////////////////// CELL /////////////////////////////

/**
* each cell has a position described by a Coordinate struct,
  a value described by a CellVal enumuration, and a list of 
  remaining values to use in the backtracking algorithm
*/
#[derive(Debug, Clone)]
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

    pub fn to_empty_cell(&mut self) {
        self.val = CellVal::None;
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CellVal {
    None,
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
            CellVal::None => String::from("_"),
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

    pub fn new(val: usize) -> CellVal {
        match val {
            1 => CellVal::One,
            2 => CellVal::Two,
            3 => CellVal::Three,
            4 => CellVal::Four,
            5 => CellVal::Five,
            6 => CellVal::Six,
            7 => CellVal::Seven,
            8 => CellVal::Eight,
            9 => CellVal::Nine,
            _ => CellVal::None,
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
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    i: usize,
    j: usize,
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Result<Coord, Error> {
        // usize can not be negative
        if i > 9 || j > 9 || i == 0 || j == 0 {
            Result::Err(Error::bounds_error())
        } else {
            Result::Ok(Coord { i, j })
        }
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
