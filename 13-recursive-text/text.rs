use std::io;

fn reverse<'a>(output: &'a mut String, input: &String, n: i32) -> &'a mut String {
  println!("{:?} {:?} {:?}", n, input.chars(), input.chars().nth(0));
  if n == 0 {
    output.push(input.chars().nth(0).unwrap());
    return output;
  }

  output.push(input.chars().nth(n as usize).unwrap());
  reverse(output, input, n - 1)
}

fn main() {
  println!("Enter some text:");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
  input = input.trim().to_string();

  let mut output = String::new();
  println!("{}", reverse(&mut output, &input, input.len() as i32));
}