pub fn solution() -> String {
  let n: u64 = 600_851_475_143;
  let mut n_divided = n;
  let mut factors: Vec<u64> = vec![];
  while n_divided % 2 == 0 {
    factors.push(2);
    n_divided /= 2;
  }
  let mut maybe_prime = 3;
  let mut max_factor = (n as f32).sqrt().floor() as u64;
  while n_divided > 1 && maybe_prime <= max_factor {
    if is_prime(maybe_prime) {
      if n_divided % maybe_prime == 0 {
        factors.push(maybe_prime);
        n_divided /= maybe_prime;
        while n_divided % maybe_prime == 0 {
          factors.push(maybe_prime);
          n_divided /= maybe_prime;
        }
        max_factor = (n as f32).sqrt().floor() as u64;
      }
    }
    maybe_prime += 2;
  }
  if n_divided > 1 {
    factors.push(n_divided);
  }
  format!("{:?}", factors)
}
