fn is_palindromic(num: i32) -> bool {
  let str_num = num.to_string();
  str_num == str_num.chars().rev().collect::<String>()
}

pub fn solution() -> String {
  let mut highest = 0;
  let mut pair = (0, 0);
  for m in (100..=999).rev() {
    for n in (100..=m).rev() {
      let product = m * n;
      if product <= highest {
        break;
      }
      if is_palindromic(product) {
        highest = product;
        pair = (m, n);
      }
    }
  }
  format!(
    "{}, which is the product of {} and {}",
    highest, pair.0, pair.1
  )
}
