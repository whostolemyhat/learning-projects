//! Contains game state and the store, and related reducer functions

use board::{ Board, Pieces };

/// The store - all game data is saved here and is read-only
/// The only way to change this data is through a reducer function
pub struct Store {
    pub state: State,
    reducer: fn(&State, Action) -> State
}

impl Store {
    pub fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            reducer: reducer
        }
    }

    /// Trigger an update on the store
    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
    Draw
}

// Current state of the game
/// - board: all pieces
/// - status: is the game in progress or has the player won or lost
/// - winner: the token of the winning player
#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub status: GameStatus,
    pub winner: Pieces
}

impl State {
    pub fn default() -> Self {
        State {
            board: Board::new(),
            status: GameStatus::Playing,
            winner: Pieces::Empty
        }
    }
}

/// Actions which can be dispatched to updated the state
#[derive(Clone)]
pub enum Action {
    BoardUpdate(BoardAction),
    Status(StatusAction),
    Winner(WinnerAction)
}

/// Reducers take a slice of state data and an action, then return a new state slice
/// with updated data
pub fn reducer(state: &State, action: Action) -> State {
    State {
        board: board_reducer(&state.board, &action),
        status: status_reducer(&state.status, &action),
        winner: winner_reducer(&state.winner, &action)
    }
}

/// Board actions and reducer just add pieces to the board
#[derive(Clone)]
pub enum BoardAction {
    Update(u8, u8, Pieces)
}

fn board_reducer(state: &Board, action: &Action) -> Board {
    let mut new_board: Board = state.clone();

    match *action {
        Action::BoardUpdate(ref board_action) => match *board_action {
            BoardAction::Update(x, y, ref token) => {
                new_board.update(x, y, token);
            },
        },
        _ => ()
    }

    new_board
}

/// Game action/reducer changes the status (is game in progress or not)
#[derive(Clone)]
pub enum StatusAction {
    Update(GameStatus)
}

fn status_reducer(state: &GameStatus, action: &Action) -> GameStatus {
    let mut new_status: GameStatus = state.clone();

    match *action {
        Action::Status(ref status_action) => match *status_action {
            StatusAction::Update(ref status) => {
                new_status = status.clone();
            }
        },
        _ => ()
    }

    new_status
}

/// Winner action/reducer contains the token of the winning player
#[derive(Clone)]
pub enum WinnerAction {
    Update(Pieces)
}

fn winner_reducer(winner: &Pieces, action: &Action) -> Pieces {
    let mut new_winner: Pieces = winner.clone();

    match *action {
        Action::Winner(ref winner_action) => match *winner_action {
            WinnerAction::Update(ref token) => {
                new_winner = token.clone();
            }
        },
        _ => ()
    }

    new_winner
}
