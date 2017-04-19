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
    pub board: Vec<Vec<Pieces>>,
    pub finished: bool,
    pub winner: Pieces
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: vec![
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty],
                vec![Pieces::Empty, Pieces::Empty, Pieces::Empty]
            ],
            winner: Pieces::Empty,
            finished: false
        }
    }

    pub fn update(&mut self, x: u8, y: u8) {
        if self.can_place(x, y) {
            self.board[x as usize][y as usize] = Pieces::Player;
        }
    }

    pub fn check_neighbours(&mut self) {
        if self.check_top() {
            let token = self.board[0][0].clone();
            self.winner(token);
        } else if self.check_centre() {
            let token = self.board[1][1].clone();
            self.winner(token);
        } else if self.check_left() {
            let token = self.board[1][0].clone();
            self.winner(token);
        } else if self.check_right() {
            let token = self.board[1][2].clone();
            self.winner(token);
        } else if self.check_bottom() {
            let token = self.board[2][1].clone();
            self.winner(token);
        }
    }

    pub fn get_status(&self) -> bool {
        self.finished
    }

    pub fn get_winner(&self) -> Pieces {
        self.winner.clone()
    }

    fn check_top(&self) -> bool {
        self.board[0][0] != Pieces::Empty && (self.board[0][0] == self.board[0][2] && self.board[0][0] == self.board[0][1])
    }

    fn check_centre(&self) -> bool {
        let vert = self.board[1][1] != Pieces::Empty && (self.board[1][1] == self.board[0][1] && self.board[1][1] == self.board[2][1]);
        let horz = self.board[1][1] != Pieces::Empty && (self.board[1][1] == self.board[1][0] && self.board[1][1] == self.board[1][2]);
        let right_diag = self.board[1][1] != Pieces::Empty && (self.board[1][1] == self.board[0][0] && self.board[1][1] == self.board[2][2]);
        let left_diag = self.board[1][1] != Pieces::Empty && (self.board[1][1] == self.board[0][2] && self.board[1][1] == self.board[2][0]);

        vert || horz || right_diag || left_diag
    }

    fn check_left(&self) -> bool {
        self.board[1][0] != Pieces::Empty && self.board[1][0] == self.board[0][0] && self.board[1][0] == self.board[2][0]
    }

    fn check_right(&self) -> bool {
        self.board[1][2] != Pieces::Empty && self.board[1][2] == self.board[0][2] && self.board[1][2] == self.board[2][2]
    }

    fn check_bottom(&self) -> bool {
        self.board[2][1] != Pieces::Empty && self.board[2][1] == self.board[2][0] && self.board[2][1] == self.board[2][2]
    }

    fn can_place(&self, x: u8, y: u8) -> bool {
        self.board[x as usize][y as usize] == Pieces::Empty
    }

    fn winner(&mut self, token: Pieces) {
        self.finished = true;
        self.winner = token;
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
