fn is_divisible(n: i32) -> bool {
  for divisor in 11..=18 {
    if n % divisor > 0 {
      return false;
    }
  }
  true
}

pub fn solution() -> String {
  let mut n = 20 * 19;
  while !is_divisible(n) {
    n += 380;
  }
  format!("{}", n)
}
