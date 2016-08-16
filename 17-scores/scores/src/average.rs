pub fn median(data: &Vec<i16>) -> f64 {
  // sort vec - in-place so clone data
  let mut sorted_data = data.clone();
  sorted_data.sort();
  // find middle
  let size = sorted_data.len();

  if size == 0 {
    return 0.0;
  }

  let middle = size / 2;

  if size % 2 == 0 {
    (sorted_data.get(middle).unwrap() + sorted_data.get(middle - 1).unwrap()) as f64 / 2f64
  } else {
    // returns a point &i16 so deref to be able to cast
    *sorted_data.get(middle).unwrap() as f64
  }
}