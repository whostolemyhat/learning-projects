extern crate rand;

mod board;
mod store;

use rand::Rng;
use std::io;

use board::{ Board, Pieces };
use store::{ Store, State, GameStatus, BoardAction, WinnerAction, StatusAction, reducer };
use store::Action::{ BoardUpdate, Winner, Status };

fn winner(store: &mut Store, token: Pieces) {
    store.dispatch(Winner(WinnerAction::Update(token.clone())));

    if token == Pieces::Player {
        store.dispatch(Status(StatusAction::Update(GameStatus::Won)));
    } else {
        store.dispatch(Status(StatusAction::Update(GameStatus::Lost)));
    }
}

fn check_neighbours(store: &mut Store) {
    // TODO tidy this up
    if check_top(&store.state.board) {
        let token = store.state.board.board[0][0].clone();
        winner(store, token);
    } else if check_centre(&store.state.board) {
        let token = store.state.board.board[1][1].clone();
        winner(store, token);
    } else if check_left(&store.state.board) {
        let token = store.state.board.board[1][0].clone();
        winner(store, token);
    } else if check_right(&store.state.board) {
        let token = store.state.board.board[1][2].clone();
        winner(store, token);
    } else if check_bottom(&store.state.board) {
        let token = store.state.board.board[2][1].clone();
        winner(store, token);
    }
}

fn check_top(board: &Board) -> bool {
    board.board[0][0] != Pieces::Empty && (board.board[0][0] == board.board[0][2] && board.board[0][0] == board.board[0][1])
}

fn check_centre(board: &Board) -> bool {
    let vert = board.board[1][1] != Pieces::Empty && (board.board[1][1] == board.board[0][1] && board.board[1][1] == board.board[2][1]);
    let horz = board.board[1][1] != Pieces::Empty && (board.board[1][1] == board.board[1][0] && board.board[1][1] == board.board[1][2]);
    let right_diag = board.board[1][1] != Pieces::Empty && (board.board[1][1] == board.board[0][0] && board.board[1][1] == board.board[2][2]);
    let left_diag = board.board[1][1] != Pieces::Empty && (board.board[1][1] == board.board[0][2] && board.board[1][1] == board.board[2][0]);

    vert || horz || right_diag || left_diag
}

fn check_left(board: &Board) -> bool {
    board.board[1][0] != Pieces::Empty && board.board[1][0] == board.board[0][0] && board.board[1][0] == board.board[2][0]
}

fn check_right(board: &Board) -> bool {
    board.board[1][2] != Pieces::Empty && board.board[1][2] == board.board[0][2] && board.board[1][2] == board.board[2][2]
}

fn check_bottom(board: &Board) -> bool {
    board.board[2][1] != Pieces::Empty && board.board[2][1] == board.board[2][0] && board.board[2][1] == board.board[2][2]
}

fn place_piece(pos: &str, mut store: &mut Store, token: Pieces) {
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
        store.dispatch(BoardUpdate(BoardAction::Update(x, y - 1, token)));
    }

    print_board(&store.state);
    check_neighbours(&mut store);
}

fn print_board(state: &State) {
    println!("{}", state.board);
}

fn print_winner(state: &State) {
    println!("Game over! {:?}", state.status);
}

fn take_turn(mut store: &mut Store) {
    // get input
    println!("Enter position: ");
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Failed to read input");

    match command.trim().len() {
        2 => place_piece(&command, &mut store, Pieces::Player),
        _ => println!("I'm sorry, I don't understand {}", command)
    }

    if store.state.board.has_space() {
        place_ai(&mut store);
    } else {
        // check if someones won yet otherwise
        println!("Game over! It's a draw");
    }
}

fn place_ai(mut store: &mut Store) {
    let mut pos = choose_space();
    while !store.state.board.can_place(pos.0, pos.1 - 1) {
        pos = choose_space();
    }

    // TODO: this is silly
    let row_names = vec!["a", "b", "c"];
    let x = row_names[pos.0 as usize];
    let pos_string = format!("{}{}", x, pos.1);

    println!("{} {}", pos_string, store.state.board.can_place(pos.0, pos.1 - 1));
    place_piece(&pos_string, &mut store, Pieces::AI);
}

fn choose_space() -> (u8, u8) {
    let x: u8 = rand::thread_rng().gen_range(0, 3);
    let y: u8 = rand::thread_rng().gen_range(1, 4);
    (x, y)
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.dispatch(Status(StatusAction::Update(GameStatus::Playing)));

    print_board(&store.state);

    loop {
        if store.state.status == GameStatus::Playing {
            take_turn(&mut store);
        } else {
            break;
        }
    }

    print_winner(&store.state);
}
