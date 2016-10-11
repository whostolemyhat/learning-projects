extern crate rand;

use std::fmt::{ Display, Result, Formatter };
use std::ops::{ Add, Sub, Mul };
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
  rows: i32,
  cols: i32,
  matrix: Vec<Vec<T>>
}

impl<T> Matrix<T> where T: Copy + Default {
  pub fn new(rows: i32, cols: i32, val: T) -> Self {
    let mut _rows: Vec<Vec<T>> = Vec::new();

    for _ in 0..rows {
      let _row = vec![val; cols as usize];
      _rows.push(_row);
    }

    Matrix { rows: rows, cols: cols, matrix: _rows }
  }

  pub fn new_from_vec(data: Vec<Vec<T>>) -> Self {
    let rows = data.len() as i32;
    let cols = data[0].len() as i32;

    Matrix { rows: rows, cols: cols, matrix: data }
  }

  pub fn scalar_mul(self, multiplier: T) -> Self where T: Mul<Output=T> + Copy + Default {
    let mut new_matrix = Matrix::new(self.rows, self.cols, Default::default());

    for i in 0..self.rows {
      let row = i as usize;

      for j in 0..self.cols {
        let col = j as usize;

        new_matrix.matrix[row][col] = self.matrix[row][col] * multiplier;
      }
    }

    new_matrix
  }

  pub fn transpose(self) -> Self {
    let mut new_matrix = Matrix::new(self.cols, self.rows, Default::default());
    for i in 0..self.rows {
      let row = i as usize;

      for j in 0..self.cols {
        let col = j as usize;

        new_matrix.matrix[col][row] = self.matrix[row][col];
      }
    }

    new_matrix
  }
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

impl<T> Mul for Matrix<T> where T: Mul<Output=T> + Copy + Default + Add<Output=T> {
  type Output = Matrix<T>;

  // this is dot product
  fn mul(self, other: Matrix<T>) -> Matrix<T> {
    // other.rows must eq self.cols
    // size = self.rows x other.cols

    // http://www.freemathhelp.com/matrix-multiplication.html
    let mut new_matrix: Matrix<T> = Matrix::new(self.rows, other.cols, Default::default());

    for i in 0..self.rows {
      for j in 0..other.cols {
        let mut total: T = Default::default();
        for k in 0..other.rows {
          total = total + self.matrix[i as usize][k as usize] * other.matrix[k as usize][j as usize];
        }
        // each number in self.row * each number in other.col
        new_matrix.matrix[i as usize][j as usize] = total;
      }
    }

    new_matrix
  }
}

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

pub fn random_matrix(rows: i32, cols: i32) -> Matrix<i32> {
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

    let third_matrix = vec![
      vec![13, 9, 7, 15],
      vec![8, 7, 4, 6],
      vec![6, 4, 0, 3]
    ];
    let third = Matrix::new_from_vec(vec![
      vec![13, 9, 7, 15],
      vec![8, 7, 4, 6],
      vec![6, 4, 0, 3]
    ]);
    assert_eq!(third.rows, 3);
    assert_eq!(third.cols, 4);
    assert_eq!(third.matrix, third_matrix);
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

  #[test]
  fn mul() {
    let first = Matrix::new(3, 3, 2);
    let second = Matrix::new(3, 3, 3);
    let third = first * second;
    let matrix = vec![vec![18, 18, 18], vec![18, 18, 18], vec![18, 18, 18]];

    assert_eq!(third.matrix, matrix);

    let mut fifth = Matrix::new(2, 2, 5);
    fifth.matrix[0][0] = 1;
    fifth.matrix[0][1] = 6;
    fifth.matrix[1][0] = 3;
    fifth.matrix[1][1] = 8;

    let mut sixth = Matrix::new(2, 2, 8);
    sixth.matrix[0][0] = 2;
    sixth.matrix[0][1] = 2;
    sixth.matrix[1][0] = 9;
    sixth.matrix[1][1] = 7;

    let second_matrix = vec![vec![56, 44], vec![78, 62]];
    assert_eq!((fifth * sixth).matrix, second_matrix);

    let mut this_one = Matrix::new(2, 3, 1);
    this_one.matrix[0][1] = 2;
    this_one.matrix[0][2] = 3;
    this_one.matrix[1][0] = 4;
    this_one.matrix[1][1] = 5;
    this_one.matrix[1][2] = 6;

    let mut another = Matrix::new(3, 1, 9);
    another.matrix[1][0] = 8;
    another.matrix[2][0] = 7;

    let third_matrix = vec![vec![46], vec![118]];
    assert_eq!((this_one * another).matrix, third_matrix);

    // [3 4 2] x [13 9 7 15 = [83 63 37 75] (83 = 3x13 + 4x8 + 2x6)
    //             8 7 4 6
    //             6 4 0 3]
    let first_dot = Matrix::new_from_vec(vec![
      vec![3, 4, 2]
    ]);
    let second_dot = Matrix::new_from_vec(vec![
      vec![13, 9, 7, 15],
      vec![8, 7, 4, 6],
      vec![6, 4, 0, 3]
    ]);
    let merged = vec![vec![83, 63, 37, 75]];

    assert_eq!((first_dot * second_dot).matrix, merged);
  }

  #[test]
  fn scalar_mul() {
    let first = Matrix::new(3, 3, 2);
    let second = first.scalar_mul(3);
    let matrix = vec![vec![6, 6, 6], vec![6, 6, 6], vec![6, 6, 6]];

    assert_eq!(second.matrix, matrix);

    let third = Matrix::new(4, 2, 3.0);
    let fourth = third.scalar_mul(0.5);
    let second_matrix = vec![vec![1.5, 1.5], vec![1.5, 1.5], vec![1.5, 1.5], vec![1.5, 1.5]];

    assert_eq!(fourth.matrix, second_matrix);
  }

  #[test]
  fn transpose() {
    let first = Matrix::new_from_vec(vec![
      vec![1, 2, 3],
      vec![4, 5, 6]
    ]);

    let second = Matrix::new_from_vec(vec![
      vec![1, 4],
      vec![2, 5],
      vec![3, 6]
    ]);

    assert_eq!((first.transpose()).matrix, second.matrix);
  }
}