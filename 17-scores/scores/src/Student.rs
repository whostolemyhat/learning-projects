// add 'mod average' to main.rs to use median
use average::median;
use std::fmt;

#[derive(Debug)]
pub struct Student {
  name: String,
  midterm_score: i16,
  final_score: i16,
  pub homework: Vec<i16>
}

impl Student {
  pub fn new(name: String, midterm_score: i16, final_score: i16, homework: Vec<i16>) -> Student {
    Student {
      name: name,
      midterm_score: midterm_score,
      final_score: final_score,
      homework: homework
    }
  }

  pub fn grade(&self) -> f64 {
    self.midterm_score as f64 * 0.2 + self.final_score as f64 * 0.4 + median(&self.homework) * 0.4
  }
}

impl fmt::Display for Student {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {:.2}", self.name, self.grade())
  }
}
