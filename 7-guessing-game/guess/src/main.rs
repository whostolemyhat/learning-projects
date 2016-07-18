extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
  let num = rand::thread_rng().gen_range(1, 100);

  let mut correct = false;
  let mut guesses = 0;

  while !correct {
    println!("Enter your guess: ");
    guesses += 1;
    let mut input_text = String::new();
    io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    // Don't crash on expected error!
    let guess: i32 = match input_text.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    let difference = (num - guess).abs();

    match difference {
      0 => println!("Correct!"),
      1 ... 10 => println!("Almost!"),
      11 ... 25 => println!("Getting there."),
      26 ... 50 => println!("Nowhere near."),
      _ => println!("Nope.")
    }

    match guess.cmp(&num) {
      Ordering::Less => println!("{} is too low! Try again.", guess),
      Ordering::Greater => println!("{} is too high! Try again.", guess),
      Ordering::Equal => {
        println!("{} is the magic number.", num);
        println!("You got the magic number in {} guesses.", guesses);
        correct = true;
      }
    }
  }
}
