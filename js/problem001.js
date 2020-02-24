function solution() {
  let sum = 0;
  for (let n = 3; n <= 999; n += 3) {
    sum += n;
  }
  for (let n = 5; n <= 995; n += 5) {
    if (n % 3 !== 0) sum += n;
  }
  return sum;
}

console.log(solution());
