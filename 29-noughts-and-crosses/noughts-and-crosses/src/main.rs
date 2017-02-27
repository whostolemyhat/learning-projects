use std::fmt::{ Display, Formatter, Result };

struct Board {
    board: Vec<Vec<char>>
}

impl Board {
    fn new() -> Self {
        Board {
            board: vec![
                vec!['.', '.', '.'],
                vec!['x', '.', '.'],
                vec!['.', '.', 'o']
            ]
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                try!(write!(f, "{}", self.board[row as usize][col as usize]));
            }
            try!(write!(f, "\n"));
        }

        Ok(())
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);

    loop {
        // get input
        // place marker
    }
}
