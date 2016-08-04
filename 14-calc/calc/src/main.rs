use std::io;

fn main() {
  println!("Hello, world!");

  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  let operators = vec!["+", "-", "/", "*"];

  for op in operators {
    println!("{}", op);

    match input_text.find(op) {
      Some(i) => {
        println!("{} at {:?}", op, i);
        // split string, get inputs
        // mmatch op to func
        // profit
      },
      None => println!("Nope")
    };
  }
}
