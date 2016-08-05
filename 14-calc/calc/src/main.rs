// this tells rust to look for calc.rs or calc/mod.rs
mod calc;

use std::io;

// look in calc for Calc
// if there was a module in calc.rs, this would be use calc::modName::Calc
use calc::Calc;

fn main() {
  println!("Enter a sum, or 'q' to quit:");

  loop {
    let mut input_text = String::new();
    io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      break;
    }

    let operators = vec!["+", "-", "/", "*"];

    for op in operators {

      match input_text.find(op) {
        Some(_) => {
          // only handle one operator
          let parts: Vec<&str> = input_text.split(op).collect();

          // need at least two numbers
          if parts.len() < 2 {
            panic!("Enter two numbers!");
          }

          let x:i64 = parts[0].trim().parse().ok().expect("Enter a number!");
          let y: i64 = parts[1].trim().parse().ok().expect("Enter a number!");;

          match op {
            "+" => println!("{}", Calc::add(x, y)),
            "-" => println!("{}", Calc::subtract(x, y)),
            "/" => println!("{}", Calc::divide(x, y)),
            "*" => println!("{}", Calc::multiply(x, y)),
            _ => println!("Sorry, only addition, subtraction, multiplication and division are supported")
          };

        },
        // ignore input if it doesn't contain operator in array
        None => {}
      };
    }
  }
}
