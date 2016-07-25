// Find the Fifth root of the sum of the squares of the first 100 ODD numbers only

// nth root: x^1/a
// math.pow(25, 1/2) == sqrt(25) == 5
// math.pow(27, 1/3) == cbrt(27) == 3
fn fifth_root(x: f64) -> f64 {
  // cast 1/5 as f64 otherwise everything just resolves as 1 :(
  x.powf(1f64/5f64)
}

fn main() {
  let mut sum: i64 = 0;

  for n in 0..100 {
    if n % 2 == 0 {
      sum += n * n;
    }
  }

  println!("5th root of {}: {}", sum, fifth_root(sum as f64));
}

#[test]
fn find_root() {
  assert!(fifth_root(161700 as f64) == 11.008851258678556);
  assert!(fifth_root(25 as f64) == 1.9036539387158786);
}