extern crate regex;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;

use regex::Regex;

fn open_file(filename: &Path) -> String {
    let mut content = String::new();
    let mut file = match File::open(&filename) {
        Err(e) => panic!("Couldn't open file {:?}", e),
        Ok(file) => file
    };

    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(e) => panic!("{:?}", e)
    };

    content
}

fn main() {
    // read file or whatever
    let text = open_file(&Path::new("./emma.txt"));

    let no_punctuation = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let replaced = no_punctuation.replace_all(&text, " ");
    let mut total_words = 0;

    let mut histogram: HashMap<String, u32> = HashMap::new();
    for word in replaced.split(" ").filter(|s| !s.is_empty()) {
        *histogram.entry(word.to_lowercase()).or_insert(0) += 1;
        total_words += 1;
    }

    let mut sorted: Vec<_> = histogram.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    // println!("{:?}", sorted);

    println!("Total words: {:?}", total_words);
    println!("Unique words: {}", sorted.len());

    println!("Most used words:");
    for i in 0..20 {
        println!("{}: {}", sorted[i].0, sorted[i].1);
    }
}
