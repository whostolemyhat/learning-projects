mod todo;
mod store;

// from http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
use std::io;
// so you don't have to namespace everything
use store::Action::{ Todos, Visibility };
use store::VisibilityFilter::{ ShowActive, ShowCompleted, ShowAll };
use store::{ Store, State, reducer };
use todo::TodoAction::{ Add, Remove, Toggle };
use todo::{ Todo };


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
