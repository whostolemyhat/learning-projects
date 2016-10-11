use std::fs::File;
use std::io::{ stdin, Read, Result };

pub fn open_file(filename: &String) -> Result<String> {
  let mut content = String::new();
  let mut file = try!(File::open(filename));
  try!(file.read_to_string(&mut content));

  Ok(content)
}

pub fn read_input() -> String {
  let mut input = String::new();
  stdin()
    .read_line(&mut input)
    .expect("Enter text!");

  input
}

pub fn read_number() -> i32 {
  read_input().trim().parse().ok().expect("Enter a number")
}