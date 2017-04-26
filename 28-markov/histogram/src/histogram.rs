use std::fs::File;
use std::path::Path;
use std::io::{ BufReader, BufRead };
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Keyword {
    phrase: String,
    occurences: u32,
    percentage: f32
}

// mega slow in dev, fast in --release
pub fn process_file(filename: &Path) -> HashMap<String, u32> {
    let mut histogram: HashMap<String, u32> = HashMap::new();

    let file = match File::open(&filename) {
        Err(e) => panic!("Error reading file: {:?}", e),
        Ok(file) => file
    };
    let buffer = BufReader::new(&file);
    for line in buffer.lines() {
        match line {
            Err(e) => println!("Error reading line: {:?}", e),
            Ok(line) => append_to_histogram(&line, &mut histogram)
        }
    }

    histogram
}

pub fn keyword_occurences(word: &str, histogram: &HashMap<String, u32>, total: &u32) -> Option<Keyword> {
    match histogram.get(word) {
        Some(value) => {
            Some(Keyword {
                phrase: word.to_string(),
                occurences: *value,
                percentage: *value as f32 / *total as f32
            })
        },
        None => None
    }
}

pub fn sort_histogram(histogram: &HashMap<String, u32>) -> Vec<(&String, &u32)> {
    let mut sorted: Vec<_> = histogram.iter().collect();
    // sort inline
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    sorted
}

pub fn get_total_words(histogram: &HashMap<String, u32>) -> u32 {
    // fold === reduce
    histogram.iter().fold(0, |acc, words| acc + words.1)
}

fn append_to_histogram(text: &String, histogram: &mut HashMap<String, u32>) {
    for word in strip(&text).split(" ").filter(|s| !s.is_empty()) {
        *histogram.entry(word.to_lowercase()).or_insert(0) += 1;
    }
}

fn strip(text: &String) -> String {
    let no_punctuation = Regex::new(r"[^a-zA-Z0-9]").unwrap();

    no_punctuation.replace_all(&text, " ").to_string()
}
