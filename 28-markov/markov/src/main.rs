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

fn strip(text: &String) -> String {
    let no_punctuation = Regex::new(r"[^a-zA-Z0-9]").unwrap();

    no_punctuation.replace_all(&text, " ").to_string()
}

fn create_histogram(text: &String) -> HashMap<String, u32> {
    let mut histogram: HashMap<String, u32> = HashMap::new();

    for word in strip(&text).split(" ").filter(|s| !s.is_empty()) {
        *histogram.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    histogram
}

fn get_total_words(histogram: &HashMap<String, u32>) -> u32 {
    histogram.iter().fold(0, |acc, words| acc + words.1)
}

// fn keyword_occurences(word: String, histogram: HashMap<String, u32>) -> Result<Some, None> {
//     histogram.get(word)
// }

fn main() {
    // read file or whatever
    let text = open_file(&Path::new("./emma.txt"));

    let histogram = create_histogram(&text);
    let mut sorted: Vec<_> = histogram.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    let total_words = get_total_words(&histogram);

    println!("Occurences: {:?} ({}%)", histogram.get("emma"), *histogram.get("emma").unwrap() as f32 / total_words as f32);
    println!("Total words: {:?}", total_words);
    println!("Unique words: {}", sorted.len());

    println!("Most used words:");
    for i in 0..30 {
        println!("{}: {}", sorted[i].0, sorted[i].1);
    }
}
