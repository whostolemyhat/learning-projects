use std::fmt::{ Display, Formatter, Result };
use std::io;

// enum Pieces {
//     Player { token: 'x' },
//     AI { token: 'o' },
//     Empty { token: '.' }
// }

struct Board {
    board: Vec<Vec<char>>
}

impl Board {
    fn new() -> Self {
        Board {
            board: vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.']
            ]
        }
    }

    fn update(&mut self, x: u8, y: u8) {
        if self.can_place(x, y) {
            self.board[x as usize][y as usize] = 'x';
        }
    }

    fn can_place(&self, x: u8, y: u8) -> bool {
        self.board[x as usize][y as usize] == '.'
    }

    fn winner(&self, token: char) {
        println!("{:?} wins!", token);
    }

    fn check_neighbours(&self) {
        if self.check_top() {
            self.winner(self.board[0][0]);
        } else if self.check_centre() {
            self.winner(self.board[1][1]);
        } else if self.check_left() {
            self.winner(self.board[1][0]);
        } else if self.check_right() {
            self.winner(self.board[1][2]);
        } else if self.check_bottom() {
            self.winner(self.board[2][1]);
        }
        // println!("{:?} {:?}", self.check_top(), self.board[0][0]);
        // println!("{:?} {:?}", self.check_centre(), self.board[1][1]);
        // println!("{:?}", self.check_left());
        // println!("{:?}", self.check_right());
        // println!("{:?}", self.check_bottom());
    }

    fn check_top(&self) -> bool {
        self.board[0][0] != '.' && (self.board[0][0] == self.board[0][2] && self.board[0][0] == self.board[0][1])
    }

    fn check_centre(&self) -> bool {
        let vert = self.board[1][1] != '.' && (self.board[1][1] == self.board[0][1] && self.board[1][1] == self.board[2][1]);
        let horz = self.board[1][1] != '.' && (self.board[1][1] == self.board[1][0] && self.board[1][1] == self.board[1][2]);
        let right_diag = self.board[1][1] != '.' && (self.board[1][1] == self.board[0][0] && self.board[1][1] == self.board[2][2]);
        let left_diag = self.board[1][1] != '.' && (self.board[1][1] == self.board[0][2] && self.board[1][1] == self.board[2][0]);

        vert || horz || right_diag || left_diag
    }

    fn check_left(&self) -> bool {
        self.board[1][0] != '.' && self.board[1][0] == self.board[0][0] && self.board[1][0] == self.board[2][0]
    }

    fn check_right(&self) -> bool {
        self.board[1][2] != '.' && self.board[1][2] == self.board[0][2] && self.board[1][2] == self.board[2][2]
    }

    fn check_bottom(&self) -> bool {
        self.board[2][1] != '.' && self.board[2][1] == self.board[2][0] && self.board[2][1] == self.board[2][2]
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

fn place_piece(pos: &str, board: &mut Board) {
    // split on every character and remove any empty strings
    let position: Vec<&str> = pos.trim().split("").filter(|s| !s.is_empty()).collect();

    // check bounds
    let row_names = vec!["a", "b", "c"];
    let y = position[1].parse::<u8>().unwrap();

    if row_names.contains(&position[0]) && y < 4 && y > 0 {
        let x: u8 = match position[0] {
            "b" => 1,
            "c" => 2,
            _ => 0
        };
        board.update(x, (y - 1));
    }

    println!("{}", board);
    board.check_neighbours();

    // has anyone won?
}

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    println!("Enter position: ");

    loop {
        // get input
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read input");

        match command.trim().len() {
            2 => place_piece(&command, &mut board),
            _ => println!("I'm sorry, I don't understand {}", command)
        }
    }
}
