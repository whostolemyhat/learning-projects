// Determine how much money is in a piggy bank that contains several
// £2 coins, £1 coins, 50p coins, 20p coins, 10p coins and
// 5p coins.

use std::io;
use std::io::Write; // stdout.flush

#[derive(Debug)]
struct Coin {
  value: i32,   // monetary value in pence
  amount: i32   // number of coins
}

impl Coin {
  // &self - this is an instance method
  fn total(&self) -> i32 {
    self.value * self.amount
  }
}

fn get_input() -> i32 {
  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");
  let num: i32 = input_text.trim().parse().ok().expect("That's no number!");

  num
}

// print! needs to be flushed to display
fn print_line(text: &str) {
  print!("{}", text);
  io::stdout().flush().unwrap();
}

fn pence_to_pounds(pence: i32) -> String {
  format!("£{:.2}", pence as f32 / 100f32)
}

fn get_total(coins: Vec<Coin>) -> String {
  let mut total: i32 = 0;

  for coin in coins {
    total = total + coin.total();
  }

  pence_to_pounds(total)
}

fn main() {
  let mut coins: Vec<Coin> = Vec::new();
  // create Coin for each, add to vec
  // iterate through vec, get total (reduce)
  println!("Enter number of coins: ");
  print_line("£2: ");
  coins.push(Coin { value: 200, amount: get_input() });

  print_line("£1: ");
  coins.push(Coin { value: 100, amount: get_input() });

  print_line("50p: ");
  coins.push(Coin { value: 50, amount: get_input() });

  print_line("20p: ");
  coins.push(Coin { value: 20, amount: get_input() });

  print_line("10p: ");
  coins.push(Coin { value: 10, amount: get_input() });

  print_line("5p: ");
  coins.push(Coin { value: 5, amount: get_input() });


  println!("\nTotal: {}", get_total(coins));
}

// Use the following values to test your programme:
// one £2,
// three £1,
// five 50p coins,
// two 20p coins,
// one 10p coin
// and fifteen 5p coins.

#[test]
fn test_coins() {
  let mut coins: Vec<Coin> = Vec::new();

  coins.push(Coin { value: 200, amount: 1 });
  coins.push(Coin { value: 100, amount: 3 });
  coins.push(Coin { value: 50, amount: 5 });
  coins.push(Coin { value: 20, amount: 2 });
  coins.push(Coin { value: 10, amount: 1 });
  coins.push(Coin { value: 5, amount: 15 });

  assert!(get_total(coins) == "£8.75");
}