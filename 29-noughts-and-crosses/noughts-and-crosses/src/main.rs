extern crate rand;

mod board;
mod store;
mod ai;

use std::io;
use rand::Rng;

use board::{ Pieces };
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

fn place_piece(pos: (u8, u8), mut store: &mut Store, token: Pieces) {
    store.dispatch(BoardUpdate(BoardAction::Update(pos.0, pos.1 - 1, token)));

    print_board(&store.state);

    // does this need to be a match?
    let token: Pieces = store.state.board.check_neighbours();
    match token {
        Pieces::Empty => (),
        _ => winner(&mut store, token)
    };
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

    // expect input to be in format "a1" etc
    match command.trim().len() {
        2 => {
            // split into chars, remove any empty pieces
            let position: Vec<&str> = command.trim().split("").filter(|s| !s.is_empty()).collect();

            match position[1].parse::<u8>() {
                Ok(y) => {
                    let x: u8 = match position[0] {
                        "b" => 1,
                        "c" => 2,
                        _ => 0
                    };

                    if y > 0 && y < 4 {
                        place_piece((x, y), &mut store, Pieces::Player)
                    } else {
                        println!("Try choosing somewhere on the board");
                    }
                },
                Err(_) => println!("Make sure you enter a position on the board (eg a1)")
            }
        },
        _ => println!("I'm sorry, I don't understand {}", command)
    }

    // TODO: don't move ai until player has made a valid move
    if store.state.board.has_space() {
        ai::place(&mut store);
    } else {
        store.dispatch(Status(StatusAction::Update(GameStatus::Draw)));
    }
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.dispatch(Status(StatusAction::Update(GameStatus::Playing)));

    print_board(&store.state);

    // let the ai go first sometimes
    if rand::thread_rng().gen_range(0, 2) == 1 {
        ai::place(&mut store);
    }

    loop {
        if store.state.status == GameStatus::Playing {
            take_turn(&mut store);
        } else {
            break;
        }
    }

    print_winner(&store.state);
}
