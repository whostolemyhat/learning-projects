use std::fmt::{ Display, Formatter, Result };
use std::io;

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

    fn update(&mut self, x: &str, y: u8) {
        let first: usize;
        match x {
            "b" => { first = 1 },
            "c" => { first = 2 },
            _ => { first = 0 }
        };
        self.board[first][y as usize] = 'x';
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
    println!("Placing piece at {}", pos);
    // split on every character and remove any empty strings
    let position: Vec<&str> = pos.trim().split("").filter(|s| !s.is_empty()).collect();
    println!("{:?}", position);
    // check bounds
    let row_names = vec!["a", "b", "c"];
    let y = position[1].parse::<u8>().unwrap();

    if row_names.contains(&position[0]) && y < 4 && y > 0 {
        board.update(&position[0], (y - 1) as u8);
    //     match position[0] {
    //         "a" => board.update(&position),
    //         _ => println!("hello")
    //     }
    }

    println!("{}", board);
    // update board
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
