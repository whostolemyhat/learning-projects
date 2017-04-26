extern crate regex;
extern crate rand;

use std::fs::File;
use std::path::Path;
use std::io::{ BufReader, BufRead };
use std::collections::HashMap;
use std::env;

use rand::distributions::{ Range, IndependentSample };

#[derive(Debug)]
struct Markov {
    prefix: Vec<String>,
    dict: HashMap<Vec<String>, Vec<String>>,
    order: u8
}

fn create_markov(filename: &Path) -> Markov {
    let mut markov = Markov {
        dict: HashMap::new(),
        prefix: Vec::new(),
        order: 3
    };

    let file = match File::open(&filename) {
        Err(e) => panic!("Error reading file: {:?}", e),
        Ok(file) => file
    };

    let buffer = BufReader::new(&file);
    for line in buffer.lines() {
        match line {
            Err(e) => println!("Error reading line: {:?}", e),
            Ok(line) => {
                for word in line.trim().split(" ").filter(|s| !s.is_empty()) {
                    process_word(word, &mut markov);
                }
            }
        }
    }

    markov
}

fn process_word(word: &str, markov: &mut Markov) {
    if markov.prefix.len() < markov.order as usize {
        markov.prefix.push(word.to_string());
    } else {
        let ref mut entry = *markov.dict.entry(markov.prefix.clone()).or_insert(Vec::new());
        entry.push(word.to_string());
        markov.prefix.remove(0);
        markov.prefix.push(word.to_string())
    }
}

fn get_random_text(markov: &Markov, length: u8) -> String {
    let mut rng = rand::thread_rng();
    let max = Range::new(0, markov.dict.len());
    let index = max.ind_sample(&mut rng);

    let mut key: Vec<String> = markov.dict.keys().collect::<Vec<_>>()[index as usize].clone();
    let mut sentence = String::new();

    for word in &key {
        sentence += word.as_str();
        sentence += " ";
    }

    for _ in 0..length {
        match markov.dict.get(&key) {
            None => println!("Error getting from dict: {:?} {:?}", key, markov.dict.get(&key)),
            Some(suffix) => {
                let between = Range::new(0, suffix.len());
                let num = between.ind_sample(&mut rng);
                let word = suffix[num as usize].as_str();

                sentence += word;
                sentence += " ";

                key.remove(0);
                key.push(word.to_string());
            }
        };
    }

    sentence
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Enter a file name");
    } else {
        let ref filename = args[1];
        let mut sentence_length = 200;

        if args.len() == 3 {
            sentence_length = args[2].parse().expect("Enter a number");
        }
        // read file for now
        // TODO: file, input or url
        let markov = create_markov(&Path::new(filename));
        println!("{}", get_random_text(&markov, sentence_length));
    }
}
