#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate handlebars;

mod todo;
mod store;
mod template;

// from http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
use std::io;
use nickel::{ Nickel, HttpRouter, FormBody };
// so you don't have to namespace everything
use store::Action::{ Todos, Visibility };
use store::VisibilityFilter::{ ShowActive, ShowCompleted, ShowAll };
use store::{ Store, State, reducer };
use todo::TodoAction::{ Add, Remove, Toggle };
use todo::{ Todo };
use template::render;
use std::sync::{ Arc, Mutex };

fn main() {
    let mut store = Store::create_store(reducer);
    store.dispatch(Todos(Add("First thing".to_string())));
    store.dispatch(Todos(Add("Another thing".to_string())));
    let store_container = Arc::new(Mutex::new(store));

    let mut server = Nickel::new();

    // create a store which is consumed by thread
    let store = store_container.clone();
    server.get("/", middleware! { |_req, res|
        let store = store.lock().unwrap();
        return render(res, "./src/todos.tpl", store.get_state());
    });

    let store = store_container.clone();
    server.get("/:action/:id", middleware! { |req, res|
        let mut store = store.lock().unwrap();

        if let Ok(num) = req.param("id").unwrap().parse::<i16>() {
            match req.param("action").unwrap() {
                "toggle" => store.dispatch(Todos(Toggle(num))),
                "remove" => store.dispatch(Todos(Remove(num))),
                _ => ()
            }
        } else {
            // look for show action
            match req.param("action").unwrap() {
                "show" => {
                    match req.param("id").unwrap() {
                        "all" => store.dispatch(Visibility(ShowAll)),
                        "active" => store.dispatch(Visibility(ShowActive)),
                        "completed" => store.dispatch(Visibility(ShowCompleted)),
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        return render(res, "./src/todos.tpl", store.get_state());
    });

    let store = store_container.clone();
    server.post("/*", middleware! { |req, res|
        let mut store = store.lock().unwrap();
        let form_body = req.form_body().ok().unwrap();
        if let Some(new_todo) = form_body.get("todo") {
            if new_todo.len() > 0 {
                store.dispatch(Todos(Add(new_todo.to_string())));
            }
        }
        return render(res, "./src/todos.tpl", store.get_state());
    });

    server.listen("0.0.0.0:1234").unwrap();
}
