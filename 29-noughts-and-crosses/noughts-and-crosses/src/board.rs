use std::fmt::{ Display, Formatter, Result };

#[derive(PartialEq, Clone, Debug)]
pub enum Pieces {
    Player,
    AI,
    Empty
}

impl Display for Pieces {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Pieces::Player => try!(write!(f, "x")),
            &Pieces::AI => try!(write!(f, "o")),
            &Pieces::Empty => try!(write!(f, "."))
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Vec<Pieces>>
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: vec![
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty]
            ]
        }
    }

    pub fn update(&mut self, x: u8, y: u8, token: &Pieces) {
        if self.can_place(x, y) {
            self.board[x as usize][y as usize] = token.clone();
        }
    }

    pub fn has_space(&self) -> bool {
        let mut space = false;
        for row in &self.board {
            if row.contains(&Pieces::Empty) {
                space = true;
                break;
            }
        };

        space
    }

    pub fn can_place(&self, x: u8, y: u8) -> bool {
        self.board[x as usize][y as usize] == Pieces::Empty
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let row_names = vec!['a', 'b', 'c'];

        try!(write!(f, "  "));
        for col in 0..self.board[0].len() {
            try!(write!(f, " {} ", col + 1));
        }

        try!(write!(f, "\n"));

        for row in 0..self.board.len() {
            try!(write!(f, "{} ", row_names[row]));

            for col in 0..self.board[row].len() {
                try!(write!(f, " {} ", self.board[row as usize][col as usize]));
            }
            try!(write!(f, "\n"));
        }

        Ok(())
    }
}
