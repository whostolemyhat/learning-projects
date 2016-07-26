// List all possible combinations of letters in a 4-letter word.
// Eg 'TEST' can be unscrambled as TEST,TETS,TSET,TSTE,TTSE,TTES

use std::io;

/// Remove the first letter
/// Find all the permutations of the remaining letters (recursive step)
/// Reinsert the letter that was removed in every possible location.
fn permutations(word: String) -> Vec<String> {
  let length = word.len();

  if length <= 1 {
    // need return keyword for early return
    // otherwise error: expected () found String
    return vec![word];
  }

  // remove first character
  let trimmed = word.chars().skip(1).collect();

  // find all permutations of remaining letters
  let perms = permutations(trimmed);
  let current_char = word.chars().nth(0).unwrap();
  let mut result = Vec::new();

  // reinsert first letter in every possible place
  for perm in &perms {
    for i in 0..&perms.len() + 1 {
      let front: String = perm.chars().take(i).collect();
      let rest: String = perm.chars().skip(i).collect();
      result.push(format!("{}{}{}", front, current_char, rest));
    }
  }


  result
}

fn main() {
  // get input
  println!("Enter a word (max 6 letters): ");

  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");

  input_text = input_text.trim().to_string();

  // check length
  let len = input_text.len();
  if len > 6 {
    panic!("Input too long!");
  }

  // print number of permutations
  let total_perms = permutations(input_text);

  // sort/dedup are in-place
  let mut unique_perms = total_perms.clone();

  // no chaining
  unique_perms.sort();
  unique_perms.dedup();
  println!("Permutations: \n{:?}\n", total_perms);
  println!("Distinct permutations: \n{:?}\n", unique_perms);
  println!("Total permutations: {}", total_perms.len());
  println!("Distinct permutations: {}", unique_perms.len());
}


#[test]
fn find_permutations() {
  assert!(permutations("bar".to_string()) == ["bar", "abr", "arb", "bra", "rba", "rab"]);
}