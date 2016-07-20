use std::io;

fn fib(target: u64) -> String {
  let mut output_string = String::new();

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
      // println!("{}", output);
      output_string = format!("{} {}", output_string, output);
    }
    i += 1;
  }

  output_string.trim().to_string()
}

fn main() {
  println!("Enter a number:");

  let mut input_text = String::new();

  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  let target: u64 = input_text.trim().parse().expect("That's no number!");

  let sequence = fib(target);

  println!("{}", sequence);
}


#[test]
fn it_calculates_sequence() {
  assert!(fib(5) == "0 1 1 2 3");
  assert!(fib(50) == "0 1 1 2 3 5 8 13 21 34");
  assert!(fib(0) == "");
}