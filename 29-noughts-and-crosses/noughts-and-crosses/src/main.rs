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
                vec!['.', 'o', '.'],
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

    fn check_neighbours(&self) {
        // top middle
        // centre
        // left middle
        // right middle
        // bottom centre

        // let token = self.board[0][1];
        // println!("looking for {}", token);
        println!("{:?}", self.board[0][0] != '.' && self.board[0][0] == self.board[0][2] && self.board[0][0] == self.board[0][1]);

        // centre
        // vert
        println!("{:?}", self.board[1][1] != '.' && self.board[1][1] == self.board[0][1] && self.board[1][1] == self.board[2][1]);
        // horz
        println!("{:?}", self.board[1][1] != '.' && self.board[1][1] == self.board[1][0] && self.board[1][1] == self.board[1][2]);
        // top-left bottom-right
        println!("{:?}", self.board[1][1] != '.' && self.board[1][1] == self.board[0][0] && self.board[1][1] == self.board[2][2]);
        // top-right bottom-left
        println!("{:?}", self.board[1][1] != '.' && self.board[1][1] == self.board[0][2] && self.board[1][1] == self.board[2][0]);

        // left middle
        println!("{:?}", self.board[1][0] != '.' && self.board[1][0] == self.board[0][0] && self.board[1][0] == self.board[2][0]);

        // right middle
        println!("{:?}", self.board[1][2] != '.' && self.board[1][2] == self.board[0][2] && self.board[1][2] == self.board[2][2]);

        // bottom centre
        println!("{:?}", self.board[2][1] != '.' && self.board[2][1] == self.board[2][0] && self.board[2][1] == self.board[2][2]);

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
