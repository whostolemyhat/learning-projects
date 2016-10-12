// https://doc.rust-lang.org/std/ops/
// https://users.rust-lang.org/t/beginner-query-implementing-traits-for-generic-structs/2871/4
extern crate matrix;
extern crate util;

use std::io;
use std::io::prelude::*;
use matrix::{ Matrix, random_matrix };
use util::read_number;

fn get_user_matrix() -> Result<Matrix<i32>, &'static str> {
  println!("Enter matrix row by row. Press 'enter' twice to finish");

  let mut matrix: Vec<Vec<i32>> = Vec::new();
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    // read line by line
    // can't do it in one go - line.unwrap() doesn't live long enough
    let tmp = line.unwrap();
    if tmp == "" {
      break;
    }

    let split_line: Vec<&str> = tmp.split(" ").collect();
    let mut row: Vec<i32> = Vec::new();
    for num in split_line {
      row.push(num.parse().ok().expect("Enter a number!"));
    }
    matrix.push(row);
  }

  if matrix.len() > 0 {
    // if uneven input, new_from_vec just takes len of first row
    let created = Matrix::new_from_vec(matrix);

    Ok(created)
  } else {
    Err("Enter the matrix")
  }

}

fn main() {
  println!("1. Enter matrices manually\n2. Use random matrices");
  let input_type = read_number();
  let mut first: Matrix<i32>;
  let mut second: Matrix<i32>;

  if input_type == 1 {
    // read in user matrix
    first = match get_user_matrix() {
      Err(e) => panic!("{:?}", e),
      Ok(matrix) => matrix
    };

    second = match get_user_matrix() {
      Err(e) => panic!("{:?}", e),
      Ok(matrix) => matrix
    };
  } else {
    println!("Enter rows:");
    let rows: i32 = read_number();

    println!("Enter cols:");
    let cols: i32 = read_number();

    first = random_matrix(rows, cols);
    second = random_matrix(rows, cols);
  }

  println!("Choose operation:\n1. Add\n2. Subtract,\n3. Multiply,\n4. Scalar multiply,\n5. Transpose");
  let choice = read_number();

  // clone's fine since we're just printing
  match choice {
    1 => println!("{}\n + \n{} \n=\n {}", first.clone(), second.clone(), first + second),
    2 => println!("{}\n - \n{} \n=\n {}", first.clone(), second.clone(), first - second),
    3 => println!("{} \n* \n{}\n =\n {}", first.clone(), second.clone(), first * second),
    4 => println!("{}\n *\n 2\n =\n {}", first.clone(), first.scalar_mul(2)),
    5 => println!("{}", first.transpose()),
    _ => println!("Choose from options 1-5")
  }
}
