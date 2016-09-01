extern crate rustc_serialize;

use std::fs::File;
use std::io;
use std::io::{ Read, Result };
use std::env;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable)]
struct Question {
  question: String,
  answer: String,
  user_answer: Option<String>
}

fn check_answer(actual_answer: &String, answer: String) -> bool {
  actual_answer.trim().to_lowercase() == answer.trim().to_lowercase()
}

// returns result since we're using try!
fn open_file(filename: &String) -> Result<String> {
  let mut content = String::new();
  let mut file = try!(File::open(filename));
  try!(file.read_to_string(&mut content));

  Ok(content)
}

fn get_questions(filename: &String) -> Vec<Question> {
  let questions = match open_file(filename) {
    Err(e) => panic!("Failed to read file: {:?}", e),
    Ok(result) => {
      let questions: Vec<Question> = match json::decode(&result) {
        Err(e) => panic!("Error parsing json: {}", e),
        Ok(quiz) => quiz
      };

      questions
    }
  };

  questions
}

fn ask_question(question: &Question) -> String {
  println!("{}", question.question);
  let mut answer = String::new();
  io::stdin()
    .read_line(&mut answer)
    .expect("Enter your answer!");

  answer
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Provide a quiz json file!");
    return;
  }

  let questions = get_questions(&args[1]);
  let mut score = 0;
  let total = questions.len();

  for question in questions {
    let answer = ask_question(&question);
    match check_answer(&question.answer, answer) {
      true => {
        println!("Correct!");
        score = score + 1;
      },
      false => {
        println!("Nope! The answer is {}", question.answer);
      }
    };
  }

  println!("You scored {} out of {}", score, total);
}

#[test]
fn it_checks_answers() {
  assert!(check_answer(&"Test".to_string(), "Test".to_string()) == true);
  assert!(check_answer(&"Test".to_string(), "test".to_string()) == true);
  assert!(check_answer(&"Test".to_string(), "test2".to_string()) == false);
}