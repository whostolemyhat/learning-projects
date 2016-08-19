// Write a programme which will print all the pairs of prime numbers
// whose sum equals the number entered by the user.

use std::io;

// http://stackoverflow.com/a/1801446/345078
fn is_prime(n: i32) -> bool {
  if n < 4 {
    return true;
  }

  if n % 2 == 0 {
    return false;
  }

  if n % 3 == 0 {
    return false;
  }

  // every prime takes the form 6k +/- 1
  let mut i = 5;
  let mut w = 2;

  while i * i <= n {
    if n % i == 0 {
      return false;
    }

    i = i + w;
    w = 6 - w;
  }

  true
}

fn main() {
  let mut input = String::new();

  println!("Enter a number:");

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

  // halve number
  // for 1 to number
  // is i prime? total - i and check prime : else skip
  let target: i32 = input.trim().parse().expect("That's no number!");

  println!("{:?}", is_prime(target));

  for i in 1..target / 2 {
    if is_prime(i) {
      if is_prime(target - i) {
        println!("{:?} {:?}", i, target - i);
      }
    }
  }

  // make a sieve of primes up to halfway and iterate through
}
