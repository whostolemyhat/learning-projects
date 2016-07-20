use std::io;

fn main() {
  println!("Enter a number:");

  let mut input_text = String::new();

  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  let target: u64 = input_text.trim().parse().expect("That's no number!");

  let mut first = 0;
  let mut second = 1;
  let mut output = 0;
  let mut i = 0;

  while output < target {
    if i <= 1 {
      output = i;
    } else {
      output = first + second;

      first = second;
      second = output;
    }

    if output < target {
      println!("{}", output);
    }
    i += 1;
  }
}
