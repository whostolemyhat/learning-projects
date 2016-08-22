// Write a programme which will print all the pairs of prime numbers
// whose sum equals the number entered by the user.

use std::io;

// http://stackoverflow.com/a/1801446/345078
fn is_prime(n: u64) -> bool {
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

fn create_sieve(limit: u64) -> Vec<bool> {
  // sieve of eratosthenes
  // create list from 2..limit
  // let p = 2 (smallest prime)
  // enumerate multiples of p up to limit ie 2p, 3p, 4p... and mark as not prime
  // p = first number not marked; if none then return list

  // vec with default vals
  let mut x = vec![true; (limit + 1) as usize];
  x[0] = false;
  x[1] = false;
  let len = x.len();

  for i in 2..len {
    if x[i as usize] == true {
      let step = i;

      // start at square instead of 2p (will have already been checked)
      let mut n = i * i;
      while n < len {
        x[n as usize] = false;

        n = n + step;
      }

      // this would be handy D:
      // for n in (i*i)..limit.step_by(i) {
        // x[n as usize] = false;
      // }
    }
  }

  x
}

fn main() {
  let mut input = String::new();
  println!("Enter a number:");

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

  let target: u64 = input.trim().parse().expect("That's no number!");
  let mut results: Vec<(u64, u64)> = Vec::new();

  // two methods: 1. walk through list of numbers to halfway
  // and check if they're prime
  for i in 2..target / 2 {
    if is_prime(i) {
      if is_prime(target - i) {;
        results.push((i, target - i));
      }
    }
  }
  println!("{:?}", results);

  let mut sieve_results: Vec<(u64, u64)> = Vec::new();
  // 2. make a sieve of primes and iterate through
  let sieve = create_sieve(target);
  for i in 1..target / 2 {
    if sieve[i as usize] == true {
      if sieve[(target - i) as usize] == true {
        sieve_results.push((i, target - i));
      }
    }
  }
  println!("{:?}", sieve_results);


  assert!(results == sieve_results);
}

#[test]
fn test_sieve() {
  let sieve = create_sieve(32);

  assert!(sieve[2] == true);
  assert!(sieve[3] == true);
  assert!(sieve[4] == false);
  assert!(sieve[5] == true);
  assert!(sieve[6] == false);
  assert!(sieve[7] == true);
  assert!(sieve[8] == false);
  assert!(sieve[9] == false);
  assert!(sieve[10] == false);
  assert!(sieve[11] == true);
  assert!(sieve[12] == false);
  assert!(sieve[13] == true);
  assert!(sieve[14] == false);
  assert!(sieve[15] == false);
  assert!(sieve[16] == false);
  assert!(sieve[17] == true);
  assert!(sieve[18] == false);
  assert!(sieve[19] == true);
  assert!(sieve[20] == false);
  assert!(sieve[21] == false);
  assert!(sieve[22] == false);
  assert!(sieve[23] == true);
  assert!(sieve[24] == false);
  assert!(sieve[25] == false);
  assert!(sieve[26] == false);
  assert!(sieve[27] == false);
  assert!(sieve[28] == false);
  assert!(sieve[29] == true);
  assert!(sieve[30] == false);
  assert!(sieve[31] == true);
  assert!(sieve[32] == false);
}

#[test]
fn test_prime() {
  assert!(is_prime(2) == true);
  assert!(is_prime(3) == true);
  assert!(is_prime(4) == false);
  assert!(is_prime(8) == false);
  assert!(is_prime(11) == true);
  assert!(is_prime(13) == true);
  assert!(is_prime(14) == false);
  assert!(is_prime(20) == false);
  assert!(is_prime(29) == true);
}