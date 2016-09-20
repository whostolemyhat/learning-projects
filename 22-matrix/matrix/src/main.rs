// https://doc.rust-lang.org/std/ops/
// https://users.rust-lang.org/t/beginner-query-implementing-traits-for-generic-structs/2871/4
extern crate rand;

use std::fmt::{ Display, Result, Formatter };
use std::ops::{ Add, Sub, Mul };
use rand::Rng;

// TOOD: generic type
#[derive(Debug, Clone)]
struct Matrix<T> {
  rows: i32,
  cols: i32,
  matrix: Vec<Vec<T>>
}

impl<T> Matrix<T> where T: Copy {
  fn new(rows: i32, cols: i32, val: T) -> Self {
    let mut _rows: Vec<Vec<T>> = Vec::new();

    for _ in 0..rows {
      let _row = vec![val; cols as usize];
      _rows.push(_row);
    }

    Matrix { rows: rows, cols: cols, matrix: _rows }
  }

  // fn new_random(rows: i32, cols: i32) -> Self {
  //   let mut _rows: Vec<Vec<i32>> = Vec::new();

  //   for _ in 0..rows {
  //     let mut _row: Vec<i32> = Vec::new();

  //     for _ in 0..cols {
  //       let num: i32 = rand::thread_rng().gen_range(-20, 20);
  //       _row.push(num);
  //     }

  //     _rows.push(_row);
  //   }

  //   Matrix { rows: rows, cols: cols, matrix: _rows }
  // }
}

// Make sure generic type T implements Add (so you can add them together)
// Copy so we can copy self.rows/self.cols to new matrix
// and Default, so we can use that to fill the matrix
// <Output=T> ensures the T implementation returns a T
impl<T> Add for Matrix<T> where T: Add<Output=T> + Copy + Default {
  type Output = Matrix<T>;

  fn add(self, other: Matrix<T>) -> Matrix<T> {
    let mut new_matrix = Matrix::new(self.rows, self.cols, Default::default());

    for i in 0..self.rows {
      let row = i as usize;
      for j in 0..self.cols {
        let col = j as usize;
        new_matrix.matrix[row][col] = self.matrix[row][col] + other.matrix[row][col];
      }
    }

    new_matrix
  }
}

impl<T> Sub for Matrix<T> where T: Sub<Output=T> + Copy + Default {
  type Output = Matrix<T>;

  fn sub(self, other: Matrix<T>) -> Matrix<T> {
    let mut new_matrix: Matrix<T> = Matrix::new(self.rows, self.cols, Default::default());

    for i in 0..self.rows {
      let row = i as usize;

      for j in 0..self.cols {
        let col = j as usize;

        new_matrix.matrix[row][col] = self.matrix[row][col] - other.matrix[row][col];
      }
    }

    new_matrix
  }
}

impl<T> Mul for Matrix<T> where T: Mul<Output=T> + Copy + Default {
  type Output = Matrix<T>;

  fn mul(self, other: Matrix<T>) -> Matrix<T> {
    // http://stattrek.com/matrix-algebra/matrix-multiplication.aspx
    let mut new_matrix: Matrix<T> = Matrix::new(other.rows, other.cols, Default::default());

    for i in 0..other.rows {
      for j in 0..other.cols {
        for k in 0..other.rows {
          new_matrix.matrix[i as usize][j as usize] = self.matrix[i as usize][k as usize] * other.matrix[k as usize][j as usize];
        }
      }
    }

    new_matrix
  }
}

// transpose
// dot
// outer

impl<T> Display for Matrix<T> where T: Display {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for row in 0..self.rows {
      for col in 0..self.cols {
        try!(write!(f, "{} ", self.matrix[row as usize][col as usize]));
      }
      try!(write!(f, "\n"));
    }

    Ok(())
  }
}

fn random_matrix(rows: i32, cols: i32) -> Matrix<i32> {
  let mut _rows: Vec<Vec<i32>> = Vec::new();

  for _ in 0..rows {
    let mut _row: Vec<i32> = Vec::new();

    for _ in 0..cols {
      let num: i32 = rand::thread_rng().gen_range(-20, 20);
      _row.push(num);
    }

    _rows.push(_row);
  }

  Matrix { rows: rows, cols: cols, matrix: _rows }
}


fn main() {
  println!("{}", random_matrix(3, 3));
  println!("{}", random_matrix(3, 6));
  println!("{}", random_matrix(2, 2));

  println!("{}", Matrix::new(3, 3, 1));
  println!("{}", Matrix::new(3, 6, 0));
  println!("{}", Matrix::new(2, 2, 2));

  let first = Matrix::new(3, 3, 1);
  let mut second = Matrix::new(3, 3, 2);
  second.matrix[1][1] = 1;
  let third = first + second;

  println!("{}", third);

  let fourth = Matrix::new(3, 3, 4);
  println!("{}", third - fourth);

  let fifth = Matrix::new(4, 4, 5);
  let sixth = Matrix::new(3, 6, 8);
  println!("{}", fifth * sixth);
}

#[cfg(test)]
mod test {
  use super::{ Matrix, random_matrix };

  #[test]
  fn create() {
    let first = Matrix::new(2, 2, 2);
    let matrix = vec![vec![2, 2], vec![2, 2]];

    assert_eq!(first.rows, 2);
    assert_eq!(first.cols, 2);
    assert_eq!(first.matrix, matrix);

    let mut second = Matrix::new(2, 4, 1);
    let second_matrix = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]];

    assert_eq!(second.rows, 2);
    assert_eq!(second.cols, 4);
    assert_eq!(second.matrix, second_matrix);

    second.matrix[1][1] = 5;
    let changed_matrix = vec![vec![1, 1, 1, 1], vec![1, 5, 1, 1]];
    assert_eq!(second.matrix, changed_matrix);

    let ran = random_matrix(2, 5);
    assert_eq!(ran.rows, 2);
    assert_eq!(ran.cols, 5);
  }

  #[test]
  fn add() {
    let first = Matrix::new(3, 3, 1);
    let second = Matrix::new(3, 3, 2);
    let third = first + second;
    let matrix = vec![vec![3, 3, 3], vec![3, 3, 3], vec![3, 3, 3]];

    assert_eq!(third.rows, 3);
    assert_eq!(third.cols, 3);
    assert_eq!(third.matrix, matrix);
  }

  #[test]
  fn sub() {
    let first = Matrix::new(3, 3, 1);
    let second = Matrix::new(3, 3, 2);
    let matrix = vec![vec![-1, -1, -1], vec![-1, -1, -1], vec![-1, -1, -1]];
    let third = first - second;

    assert_eq!(third.rows, 3);
    assert_eq!(third.cols, 3);
    assert_eq!(third.matrix, matrix);
  }
}