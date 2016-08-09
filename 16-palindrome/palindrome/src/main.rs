extern crate regex;

use std::io;
use regex::Regex;

fn is_palindrome(text: String) -> bool {
  let mut i = 0;

  for letter in text.chars() {
    if i <= text.len() / 2 {
      if letter != text.chars().rev().nth(i).unwrap() {
        return false;
      }
      i = i + 1;
    } else {
      break;
    }
  }

  true
}

fn main() {
  println!("Enter some text:");
  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  input_text = input_text.trim().to_lowercase();
  // not alpha or numeric
  let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
  // take full slice, ie convert String to str
  let stripped = re.replace_all(&input_text[..], "");

  let palindrome = is_palindrome(stripped);
  if palindrome {
    println!("Yep! {} is a palindrome.", input_text);
  } else {
    println!("Nope, {} isn't a palindrome", input_text);
  }
}
