function isPrime(n) {
  if (typeof n !== "number") throw new Error("not a number");
  if (!Number.isInteger(n)) throw new Error("must be an integer");
  if (n <= 0) return false;
  if (n <= 3) return true;
  if (n % 2 == 0 || n % 3 == 0) return false;
  const root = Math.floor(Math.sqrt(n));
  let maybePrimeBase = 6;
  while (root <= maybePrimeBase - 1) {
    if (n % (maybePrimeBase - 1) == 0) return false;
    if (n % (maybePrimeBase + 1) == 0) return false;
    maybePrimeBase += 6;
  }
  return true;
}

module.exports = { isPrime };
