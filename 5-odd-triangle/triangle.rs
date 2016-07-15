use std::io;

fn line(num: u64) -> String {
  let mut buf = String::with_capacity(num as usize);

  for _ in 0..num {
    buf.push('*');
  }

  buf
}

// don't compile for tests
#[cfg(not(test))]
fn main() {
  println!("Enter a number: ");

  let mut input_text = String::new();

  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  let num = input_text.trim().parse::<u64>().expect("That's no number");

  for i in 1..num + 1 {
    if i % 2 != 0 {
      println!("{}", line(i));
    }
  }
}

// cargo test or rustc --test
#[cfg(test)]
#[test]
fn it_works() {
  assert!(line(3) == "***");
  assert!(line(1) == "*");
}
