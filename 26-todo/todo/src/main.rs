// from http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
use std::io;
// so you don't have to namespace everything
use Action::{ Todos, Visibility };
use TodoAction::{ Add, Remove, Toggle };
use VisibilityFilter::{ ShowActive, ShowCompleted, ShowAll };

#[derive(Debug, Clone)]
struct State {
    todos: Vec<Todo>,
    filter: VisibilityFilter
}

impl State {
    fn default() -> Self {
        State {
            todos: Vec::new(),
            filter: VisibilityFilter::ShowAll
        }
    }
}

#[derive(Clone, Debug)]
enum VisibilityFilter {
    ShowActive,
    ShowAll,
    ShowCompleted
}

#[derive(Clone, Debug)]
enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter)
}

#[derive(Clone, Debug)]
enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16)
}

fn reducer(state: &State, action: Action) -> State {
    State {
        todos: todo_reducer(&state.todos, &action),
        filter: visibility_reducer(&state.filter, &action)
    }
}

fn get_mut_todo(todos: &mut Vec<Todo>, id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo| todo.id == id)
}

fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
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

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone()
    }
}

struct Store {
    state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State
}

impl Store {
    fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer
        }
    }

    fn subscribe(&mut self, listener: fn(&State)) {
        self.listeners.push(listener);
    }

    // fn get_state(&self) -> &State {
    //     &self.state
    // }

    fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    id: i16,          // unique key
    description: String,
    completed: bool,
    order: i32,          // where in the list should it be
    deleted: bool
}

impl Todo {
    fn new(id: i16, description: String, order: i32) -> Self {
        Todo {
            id: id,
            description: description,
            completed: false,
            order: order,
            deleted: false
        }
    }
}

fn print_todo(todo: &Todo) {
    let completed = if todo.completed { "✔" } else { " " };
    println!("[{}] {}. {}", completed, todo.id, todo.description);
}

fn print_todos(state: &State) {
    let visibility = &state.filter;

    println!("\nTodo list\n===================");
    println!("Filter: {:?}", visibility);
    for todo in &state.todos {
        if !todo.deleted {
            match *visibility {
                ShowAll => print_todo(&todo),
                ShowCompleted => if todo.completed {
                    print_todo(&todo)
                },
                ShowActive => if !todo.completed {
                    print_todo(&todo)
                }
            }
        }
    }

    if state.todos.len() as i16 == 0 {
        println!("(No items to display)");
    }
    println!("\n");
}

fn invalid_command(command: &str) {
    println!("Invalid command {}", command);
}

fn show_help() {
    println!("Commands:");
    println!(" help: show this message");
    println!(" add [text]: add a todo item with text");
    println!(" toggle [id]: toggle selected item completed state");
    println!(" list [all|active|completed]: filter items");
    println!(" remove [id]: delete item")
}

fn main() {
    let mut store = Store::create_store(reducer);
    store.subscribe(print_todos);
    show_help();
    print_todos(&store.state);

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input");

        let command_parts: Vec<&str> = command.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&command),
            _ => match command_parts[0] {
                "add" => store.dispatch(Todos(Add(command_parts[1..].join(" ").to_string() ))),
                "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
                    store.dispatch(Todos(Remove(num)));
                },
                "toggle" => if let Ok(num) = command_parts[1].parse::<i16>() {
                    store.dispatch(Todos(Toggle(num)));
                },
                "list" => {
                    if command_parts.len() > 1 {
                        match command_parts[1] {
                            "all" => store.dispatch(Visibility(ShowAll)),
                            "active" => store.dispatch(Visibility(ShowActive)),
                            "completed" => store.dispatch(Visibility(ShowCompleted)),
                            _ => invalid_command(&command)
                        }
                    } else {
                        store.dispatch(Visibility(ShowAll))
                    }
                },
                "help" => show_help(),
                _ => invalid_command(&command)
            }
        }
    }

}
