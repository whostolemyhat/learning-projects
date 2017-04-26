extern crate regex;
extern crate rand;

use std::fs::File;
use std::path::Path;
use std::io::{ BufReader, BufRead };
use std::collections::HashMap;
use std::env;

use rand::distributions::{ Range, IndependentSample };

mod histogram;

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


        let histogram = histogram::process_file(&Path::new(filename));
        let sorted = histogram::sort_histogram(&histogram);
        let total_words = histogram::get_total_words(&histogram);


        println!("Occurences {:?}", histogram::keyword_occurences(&"dr", &histogram, &total_words));
        println!("Occurences {:?}", histogram::keyword_occurences(&"frankenstein", &histogram, &total_words));
        println!("Total words: {:?}", total_words);
        println!("Unique words: {}", sorted.len());

        println!("Most used words:");
        for i in 0..20 {
            println!("{:?}", histogram::keyword_occurences(&sorted[i].0, &histogram, &total_words));
        }
    }
}
