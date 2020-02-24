const { isPrime } = require("./isPrime");

function solution() {
  let factors = [];
  const n = 600851475143;
  let current = n;
  if (n % 3 == 0) {
    factors.push(3);
    current /= 3;
  }
  const root = Math.floor(Math.sqrt(n));
  let maybePrime = 5;
  while (maybePrime < Math.floor(Math.sqrt(current))) {
    if (isPrime(maybePrime) && n % maybePrime == 0) {
      factors.push(maybePrime);
      current /= maybePrime;
    }
    maybePrime += 2;
    if (maybePrime <= root && isPrime(maybePrime) && n % maybePrime == 0) {
      factors.push(maybePrime);
      current /= maybePrime;
    }
    maybePrime += 4;
  }
  factors.push(current);
  return factors;
}

console.log(solution());
