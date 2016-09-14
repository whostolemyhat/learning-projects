extern crate regex;
extern crate util;

mod html;

use std::env;
use html::strip_tags;
use util::open_file;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Enter a source file as argument");
    return;
  }

  let text = match open_file(&args[1]) {
    Ok(content) => content,
    Err(e) => panic!("Error opening {}: {}", &args[1], e)
  };

  println!("{}", strip_tags(text));
}