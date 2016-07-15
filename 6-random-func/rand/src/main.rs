extern crate rand;

use rand::Rng;

fn first() -> String {
  "I'm in the first function".to_string()
}

fn second() -> String {
  "I'm the second".to_string()
}

fn third() -> String {
  "Tertiary".to_string()
}

fn main() {
  let num = rand::thread_rng().gen_range(1, 3);

  match num {
    1 => println!("{}", first()),
    2 => println!("{}", second()),
    3 => println!("{}", third()),
    _ => println!("I'm sure you won't see this")
  }
}
