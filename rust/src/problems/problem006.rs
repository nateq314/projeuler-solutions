pub fn solution() -> String {
  let mut sum_of_squares = 0;
  let mut sum = 0;
  for n in 1..=100 {
    sum += n;
    sum_of_squares += n * n;
  }
  let answer = sum * sum - sum_of_squares;
  format!("{}", answer)
}
