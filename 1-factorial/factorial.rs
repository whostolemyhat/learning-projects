use std::io;

fn factorial(num: u64) -> u64 {
  if num < 1 {
    return 0;
  }

  num + factorial(num - 1)
}

fn main() {
  println!("Enter a number: ");

  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  // todo: better error handling
  let num = input_text.trim().parse::<u64>().expect("That's not a number");

  println!("{}", factorial(num));
}