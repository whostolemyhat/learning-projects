// this basically imports the student.rs file
// equiv of import Student from student
mod student;

// use mod average here to allow Student access to median
mod average;

use std::io;
use student::Student;

fn main() {
  println!("Enter student name, midterm, final and homework results. Q to finish");

  let mut students: Vec<Student> = Vec::new();

  loop {
    let mut input_text = String::new();
    io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      break;
    }

    let line: Vec<&str> = input_text.trim().split(' ').collect();
    if line.len() > 2 {
      let (args, homework) = line.split_at(3);
      let name = args[0];
      let midterm: i16 = args[1].parse().expect("Enter midterm mark as number");
      let final_score: i16 = args[2].parse().expect("Enter final mark as number");

      // todo: handle parse error
      let homework_scores: Vec<i16> = homework.iter().map(|val| val.parse().unwrap()).collect();

      let student = Student::new(name.to_string(), midterm, final_score, homework_scores);
      students.push(student);
    } else {
      println!("Enter name and all results one one line!");
    }
  }

  for student in students {
    println!("{}", student);
  }
}
