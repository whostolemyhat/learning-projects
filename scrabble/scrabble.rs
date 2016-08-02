// http://dtrace.org/blogs/ahl/2015/06/22/first-rust-program-pain/
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::io::stdin;

fn sort_str(s: &String) -> String {
  let mut v: Vec<char> = s.chars().collect();
  v.sort();
  v.into_iter().collect()
}

fn main() {
  let path = Path::new("./dictionary.txt");
  let file = match File::open(&path) {
    Err(e) => panic!("Failed to open {}: {}", path.display(), e),
    Ok(f) => f
  };

  // for each word, create a hashmap of letters
  // in alphabetical order: original word
  // there can be more than one word per input string eg beisstu
  let mut dict: HashMap<String, Vec<String>> = HashMap::new();
  for line in BufReader::new(file).lines() {
    let s = line.unwrap();
    dict.entry(sort_str(&s)).or_insert(Vec::new()).push(s);
  }

  // println!("{:?}", dict);

  // input must be capitalised and in alphabetical order
  // eg beisstu -> busiest
  let sin = stdin();
  for line in sin.lock().lines() {
    let s = line.unwrap().to_uppercase();

    match dict.get(&sort_str(&s)) {
      Some(v) => println!("anagrams for {}: {}", s, v.join(" ")),
      None => println!("Nothing found")
    }
  }
}