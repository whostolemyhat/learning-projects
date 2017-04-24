//! Contains the AI logic

extern crate rand;

// not sure why use rand::Rng isn't happy
use self::rand::Rng;

use store::{ Store };
use board::{ Pieces };

/// Pick an empty space on the board and put a piece there
pub fn place(mut store: &mut Store) {
    let mut pos = choose_space();
    while !store.state.board.can_place(pos.0, pos.1 - 1) {
        pos = choose_space();
    }

    // TODO: bit pants coupling
    super::place_piece(pos, &mut store, Pieces::AI);
}

/// Pick a random x,y co-ord on the board
fn choose_space() -> (u8, u8) {
    let x: u8 = rand::thread_rng().gen_range(0, 3);
    let y: u8 = rand::thread_rng().gen_range(1, 4);

    (x, y)
}
