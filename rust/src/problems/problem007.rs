use super::is_prime::is_prime;

pub fn solution() -> String {
  let mut maybe_prime = 3;
  let mut n = 2;
  while n < 10001 {
    maybe_prime += 2;
    if is_prime(maybe_prime) {
      n += 1;
    }
  }
  format!("{}", maybe_prime)
}
