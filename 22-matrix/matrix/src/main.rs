extern crate rand;

use std::fmt;
use rand::Rng;

#[derive(Debug)]
struct Matrix {
  rows: i32,
  cols: i32,
  matrix: Vec<Vec<i32>>
}

impl Matrix {
  fn new(rows: i32, cols: i32) -> Self {
    // let _rows = Vec::new();
    let mut _cols: Vec<Vec<i32>> = Vec::new();

    for _ in 0..cols {
      let mut _row: Vec<i32> = Vec::new();

      for _ in 0..rows {
        let num = rand::thread_rng().gen_range(-20, 20);
        _row.push(num);
      }

      _cols.push(_row);
    }

    Matrix { rows: rows, cols: cols, matrix: _cols }
  }

  // fn new_random(rows: i32, cols: i32) -> Self {
    // Matrix {}
  // }
}

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..self.cols {
      for x in 0..self.rows {
        try!(write!(f, "{} ", self.matrix[y as usize][x as usize]));
      }
      try!(write!(f, "\n"));
    }

    Ok(())
  }
}

fn main() {
  println!("{}", Matrix::new(3, 3));
  println!("{}", Matrix::new(3, 6));
  println!("{}", Matrix::new(2, 2));
}
