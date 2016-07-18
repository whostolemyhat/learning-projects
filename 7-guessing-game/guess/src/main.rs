extern crate rand;

use rand::Rng;
use std::io;

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

    let guess = input_text.trim().parse::<i32>().expect("That's no number!");
    let difference = (num - guess).abs();

    match difference {
      0 => println!("Correct!"),
      1 ... 10 => println!("Almost!"),
      11 ... 25 => println!("Getting there."),
      26 ... 50 => println!("Nowhere near."),
      _ => println!("Nope.")
    }

    if guess < num {
      println!("{} is too low! Try again.", guess);
    } else if guess > num {
      println!("{} is too high! Try again.", guess);
    } else if guess == num {
      println!("{} is the magic number.", num);
      println!("You got the magic number in {} guesses.", guesses);
      correct = true;
    }
  }
}
