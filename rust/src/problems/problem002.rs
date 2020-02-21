pub fn solution() -> String {
  let mut m = 1;
  let mut n = 2;
  let mut sum = 0;
  while n < 4_000_000 {
    if n % 2 == 0 {
      sum += n;
    }
    let temp: u32 = n;
    n += m;
    m = temp;
  }
  format!("{}", sum)
}
