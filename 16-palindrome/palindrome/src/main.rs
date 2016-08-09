extern crate regex;

use std::io;
use regex::Regex;

fn is_palindrome(text: String) -> bool {
  let lower = text.to_lowercase();
  let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
  // take full slice, ie convert String to str
  let stripped = re.replace_all(&lower[..], "");

  let mut i = 0;

  for letter in stripped.chars() {
    if i <= stripped.len() / 2 {
      if letter != stripped.chars().rev().nth(i).unwrap() {
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

  if is_palindrome(input_text) {
    println!("Yep!");
  } else {
    println!("Nope");
  }
}

#[test]
fn check_it() {
  assert!(is_palindrome("hello".to_string()) == false);
  assert!(is_palindrome("racecar".to_string()) == true);
  assert!(is_palindrome("Race car".to_string()) == true);
  assert!(is_palindrome("Hello! Ol, le +H".to_string()) == true);
  assert!(is_palindrome("1331".to_string()) == true);
}