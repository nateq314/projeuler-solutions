function solution() {
  let sum = 0;
  let a = 1,
    b = 2;
  while (b < 4000000) {
    if (b % 2 == 0) sum += b;
    const newFib = a + b;
    a = b;
    b = newFib;
  }
  return sum;
}

console.log(solution());
