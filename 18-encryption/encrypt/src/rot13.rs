pub fn rot13(line: &String) -> String {
  let mut converted = String::new();

  for letter in line.chars() {
    if letter.is_alphabetic() {
      converted.push(get_char_code(letter));
    } else {
      converted.push(letter);
    }
  }

  converted
}

// only letters in English alphabet
// just hardcode it :)
fn get_char_code(letter: char) -> char {
  match letter {
    'A' => 'N',
    'B' => 'O',
    'C' => 'P',
    'D' => 'Q',
    'E' => 'R',
    'F' => 'S',
    'G' => 'T',
    'H' => 'U',
    'I' => 'V',
    'J' => 'W',
    'K' => 'X',
    'L' => 'Y',
    'M' => 'Z',
    'N' => 'A',
    'O' => 'B',
    'P' => 'C',
    'Q' => 'D',
    'R' => 'E',
    'S' => 'F',
    'T' => 'G',
    'U' => 'H',
    'V' => 'I',
    'W' => 'J',
    'X' => 'K',
    'Y' => 'L',
    'Z' => 'M',
    'a' => 'n',
    'b' => 'o',
    'c' => 'p',
    'd' => 'q',
    'e' => 'r',
    'f' => 's',
    'g' => 't',
    'h' => 'u',
    'i' => 'v',
    'j' => 'w',
    'k' => 'x',
    'l' => 'y',
    'm' => 'z',
    'n' => 'a',
    'o' => 'b',
    'p' => 'c',
    'q' => 'd',
    'r' => 'e',
    's' => 'f',
    't' => 'g',
    'u' => 'h',
    'v' => 'i',
    'w' => 'j',
    'x' => 'k',
    'y' => 'l',
    'z' => 'm',
    _ => ' '
  }
}
