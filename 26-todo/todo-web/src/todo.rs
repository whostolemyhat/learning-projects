extern crate rustc_serialize;

use store::{ Action };
use store::Action::{ Todos };
use todo::TodoAction::{ Add, Toggle, Remove };

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Todo {
    pub id: i16,          // unique key
    pub description: String,
    pub completed: bool,
    pub order: i32,          // where in the list should it be
    pub deleted: bool
}

impl Todo {
    pub fn new(id: i16, description: String, order: i32) -> Self {
        Todo {
            id: id,
            description: description,
            completed: false,
            order: order,
            deleted: false
        }
    }
}

#[derive(Clone, Debug)]
pub enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16)
}

pub fn get_mut_todo(todos: &mut Vec<Todo>, id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo| todo.id == id)
}

pub fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    match *action {
        Todos(ref todo_action) => match *todo_action {
            Add(ref title) => {
                let new_id = new_state.len() as i16 + 1;
                new_state.push(Todo::new(new_id, title.to_string(), 0))
            },
            Toggle(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    if todo.completed {
                        todo.completed = false;
                    } else {
                        todo.completed = true;
                    }
                }
            },
            Remove(todo_id) => {
                if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                    todo.deleted = true;
                }
            }
        },
        // not a todo action
        _ => ()
    }

    new_state
}
