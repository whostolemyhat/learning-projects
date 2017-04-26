extern crate regex;

use std::fs::File;
use std::path::Path;
use std::io::{ BufReader, BufRead, Read };
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

fn create_histogram(text: &String) -> HashMap<String, u32> {
    let mut histogram: HashMap<String, u32> = HashMap::new();

    for word in strip(&text).split(" ").filter(|s| !s.is_empty()) {
        *histogram.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    histogram
}

// mega slow in dev, fast in --release
fn process_file(filename: &Path) -> HashMap<String, u32> {
    let mut histogram: HashMap<String, u32> = HashMap::new();

    let file = match File::open(&filename) {
        Err(e) => panic!("Error reading file: {:?}", e),
        Ok(file) => file
    };
    let mut buffer = BufReader::new(&file);
    for line in buffer.lines() {
        match line {
            Err(e) => println!("Error reading line: {:?}", e),
            Ok(line) => append_to_histogram(&line, &mut histogram)
        }
    }

    histogram
}

fn strip(text: &String) -> String {
    let no_punctuation = Regex::new(r"[^a-zA-Z0-9]").unwrap();

    no_punctuation.replace_all(&text, " ").to_string()
}


fn append_to_histogram(text: &String, histogram: &mut HashMap<String, u32>) {
    for word in strip(&text).split(" ").filter(|s| !s.is_empty()) {
        *histogram.entry(word.to_lowercase()).or_insert(0) += 1;
    }
}

fn sort_histogram(histogram: &HashMap<String, u32>) -> Vec<(&String, &u32)> {
    let mut sorted: Vec<_> = histogram.iter().collect();
    // sort inline
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    sorted
}

fn get_total_words(histogram: &HashMap<String, u32>) -> u32 {
    // fold === reduce
    histogram.iter().fold(0, |acc, words| acc + words.1)
}

#[derive(Debug)]
struct Markov {
    prefix: Vec<String>,
    dict: HashMap<String, Vec<String>>,
    order: u8
}

fn create_markov(filename: &Path) -> Markov {
    // let mut dict = HashMap::new();
    // let mut prefix: Vec<str> = Vec::new();

    let mut markov = Markov {
        dict: HashMap::new(),
        prefix: Vec::new(),
        order: 2
    };

    let file = match File::open(&filename) {
        Err(e) => panic!("Error reading file: {:?}", e),
        Ok(file) => file
    };
    let mut buffer = BufReader::new(&file);
    for line in buffer.lines() {
        match line {
            Err(e) => println!("Error reading line: {:?}", e),
            Ok(line) => {
                // append_to_histogram(&line)
                for word in strip(&line).split(" ").filter(|s| !s.is_empty()) {
                    process_word(word.to_lowercase(), &mut markov);
                }
            }
        }
    }

    markov
}

fn process_word(word: String, markov: &mut Markov) {
    if markov.prefix.len() < markov.order as usize {
        markov.prefix.push(word.clone());
    } else {
        let key = markov.prefix.join(",");
        let ref mut entry = *markov.dict.entry(key).or_insert(Vec::new());
        entry.push(word.clone());
        markov.prefix.pop();
        markov.prefix.push(word.clone())
    }
}

#[derive(Debug)]
struct Keyword {
    phrase: String,
    occurences: u32,
    percentage: f32
}

fn keyword_occurences(word: &str, histogram: &HashMap<String, u32>, total: &u32) -> Option<Keyword> {
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

fn main() {
    // read file for now
    // TODO: file, input or url
    // let histogram = process_file(&Path::new("./emma.txt"));
    let histogram = create_markov(&Path::new("./emma.txt"));
    println!("{:?}", histogram);

    // let text = open_file(&Path::new("./emma.txt"));
    // let histogram = create_histogram(&text);
    // let sorted = sort_histogram(&histogram);
    // let total_words = get_total_words(&histogram);
    //
    //
    // println!("Occurences {:?}", keyword_occurences(&"emma", &histogram, &total_words));
    // println!("Occurences {:?}", keyword_occurences(&"brian", &histogram, &total_words));
    // println!("Total words: {:?}", total_words);
    // println!("Unique words: {}", sorted.len());
    //
    // println!("Most used words:");
    // for i in 0..20 {
    //     // println!("{}: {}", sorted[i].0, sorted[i].1);
    //     println!("{:?}", keyword_occurences(&sorted[i].0, &histogram, &total_words));
    // }
}
