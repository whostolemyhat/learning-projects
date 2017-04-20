use board::{ Board, Pieces };

pub struct Store {
    pub state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State
}

impl Store {
    pub fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer
        }
    }

    pub fn subscribe(&mut self, listener: fn(&State)) {
        self.listeners.push(listener);
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Won,
    Lost
}

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

#[derive(Clone)]
pub enum Action {
    BoardUpdate(BoardAction),
    Status(StatusAction),
    Winner(WinnerAction)
}

pub fn reducer(state: &State, action: Action) -> State {
    State {
        board: board_reducer(&state.board, &action),
        status: status_reducer(&state.status, &action),
        winner: winner_reducer(&state.winner, &action)
    }
}

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
