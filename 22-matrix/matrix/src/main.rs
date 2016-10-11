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
  // read in user matrix
  // match get_user_matrix() {
  //   Err(e) => panic!("{:?}", e),
  //   Ok(matrix) => println!("{}", matrix)
  // }

  println!("Enter rows:");
  let rows: i32 = read_number();

  println!("Enter cols:");
  let cols: i32 = read_number();

  let ran = random_matrix(rows, cols);
  let another = random_matrix(rows, cols);
  println!("{}\n\n{}", ran, another);

  // testing stuff
  // println!("{}", random_matrix(3, 3));
  // println!("{}", random_matrix(3, 6));
  // println!("{}", random_matrix(2, 2));

  // println!("{}", Matrix::new(3, 3, 1));
  // println!("{}", Matrix::new(3, 6, 0));
  // println!("{}", Matrix::new(2, 2, 2));

  // let first = Matrix::new(3, 3, 1);
  // let mut second = Matrix::new(3, 3, 2);
  // second.matrix[1][1] = 1;
  // let third = first + second;

  // println!("{}", third);

  // let fourth = Matrix::new(3, 3, 4);
  // println!("{}", third - fourth);

  // let mut fifth = Matrix::new(2, 3, 0);
  // fifth.matrix[0][1] = 1;
  // fifth.matrix[0][2] = 2;
  // fifth.matrix[1][0] = 3;
  // fifth.matrix[1][1] = 4;
  // fifth.matrix[1][2] = 5;

  // let mut sixth = Matrix::new(3, 2, 6);
  // sixth.matrix[0][1] = 7;
  // sixth.matrix[1][0] = 8;
  // sixth.matrix[1][1] = 9;
  // sixth.matrix[2][0] = 10;
  // sixth.matrix[2][1] = 11;

  // println!("{}", fifth * sixth);

  // let this_matrix = vec![
  //   vec![1, 2, 3],
  //   vec![4, 5, 6]
  // ];
  // let this_one = Matrix::new_from_vec(this_matrix);

  // let another_matrix = vec![
  //   vec![9],
  //   vec![8],
  //   vec![7]
  // ];
  // let another = Matrix::new_from_vec(another_matrix);

  // println!("{}", this_one * another);

  // let seventh = Matrix::new(4, 5, 7);
  // println!("{}", seventh.scalar_mul(3));

  // let first_dot = Matrix::new_from_vec(vec![
  //   vec![3, 4, 2]
  // ]);
  // let matrix = vec![
  //   vec![13, 9, 7, 15],
  //   vec![8, 7, 4, 6],
  //   vec![6, 4, 0, 3]
  // ];
  // let second_dot = Matrix::new_from_vec(matrix);
  // println!("{}", first_dot * second_dot);

  // let transpose_one = Matrix::new_from_vec(vec![
  //   vec![1, 2, 3],
  //   vec![4, 5, 6]
  // ]);
  // println!("{}", transpose_one.transpose());
}
