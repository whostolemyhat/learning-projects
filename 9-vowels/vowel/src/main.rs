use std::io;

fn main() {
  println!("Enter some text:");

  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  let vowels = vec!['a', 'e', 'i', 'o', 'u'];
  let mut vowel_count = 0;
  let mut consonant_count = 0;

  for c in input_text.chars() {
    if c.is_alphabetic() {
      if vowels.contains(&c) {
        vowel_count += 1;
      } else {
        consonant_count += 1;
      }
    }
  }

  println!("Vowel count: {}", vowel_count);
  println!("Consonant count: {}", consonant_count);
}
