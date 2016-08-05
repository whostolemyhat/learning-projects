pub struct Calc;
impl Calc {
  pub fn add(x: i64, y: i64) -> i64 {
    x + y
  }

  pub fn multiply(x: i64, y: i64) -> i64 {
    x * y
  }

  pub fn subtract(x: i64, y: i64) -> i64 {
    x - y
  }

  pub fn divide(x: i64, y: i64) -> f64 {
    x as f64 / y as f64
  }
}

#[cfg(test)]
mod test {
  use calc::Calc;

  #[test]
  fn addition() {
    assert!(Calc::add(10, 2) == 12);
    assert!(Calc::add(-10, -2) == -12);
    assert!(Calc::add(-10, 2) == -8);
  }

  #[test]
  fn multiplication() {
    assert!(Calc::multiply(10, 2) == 20);
    assert!(Calc::multiply(10, -2) == -20);
    assert!(Calc::multiply(-10, -2) == 20);
  }

  #[test]
  fn subtraction() {
    assert!(Calc::subtract(10, 2) == 8);
    assert!(Calc::subtract(2, 10) == -8);
  }

  #[test]
  fn division() {
    assert!(Calc::divide(10, 2) == 5f64);
    assert!(Calc::divide(2, 10) == 0.2);
  }
}