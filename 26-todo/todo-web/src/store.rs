extern crate rustc_serialize;

use rustc_serialize::json::{ self, Json, ToJson };
use store::Action::{ Visibility };
use todo::{ Todo, TodoAction, todo_reducer };

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

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}


#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct State {
    pub todos: Vec<Todo>,
    pub filter: VisibilityFilter
}

impl State {
    pub fn default() -> Self {
        State {
            todos: Vec::new(),
            filter: VisibilityFilter::ShowAll
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        Json::from_str(&json::encode(&self).unwrap()).unwrap()
    }
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted
}

#[derive(Clone, Debug)]
pub enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter)
}

pub fn reducer(state: &State, action: Action) -> State {
    State {
        todos: todo_reducer(&state.todos, &action),
        filter: visibility_reducer(&state.filter, &action)
    }
}

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone()
    }
}
