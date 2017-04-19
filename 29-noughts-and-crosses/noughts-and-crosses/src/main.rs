mod board;
mod store;

use std::io;

// use board::{ Board };
use store::{ Store, State, BoardAction, reducer };
use store::Action::{ BoardUpdate };

fn place_piece(pos: &str, store: &mut Store) {
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
        // board.update(x, (y - 1));
        store.dispatch(BoardUpdate(BoardAction::Update(x, y - 1)));
    }

    store.dispatch(BoardUpdate(BoardAction::Check));
    if store.state.board.get_status() {
        println!("game over! {} wins", store.state.board.get_winner());
        // store.dispatch(BoardUpdate);
    }
}

fn print_board(state: &State) {
    println!("{}", state.board);
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.subscribe(print_board);
    print_board(&store.state);

    println!("Enter position: ");

    loop {
        // get input
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read input");

        match command.trim().len() {
            2 => place_piece(&command, &mut store),
            _ => println!("I'm sorry, I don't understand {}", command)
        }
    }
}
