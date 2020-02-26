import 'dart:math';

bool isPrime(int n) {
  if (n <= 0) throw "must be a positive integer";
  if (n <= 3) return true;
  if (n % 2 == 0 || n % 3 == 0) return false;
  var root = sqrt(n).floor();
  var maybePrimeBase = 6;
  while (maybePrimeBase <= root) {
    if (n % (maybePrimeBase - 1) == 0) return false;
    if (n % (maybePrimeBase + 1) == 0) return false;
    maybePrimeBase += 6;
  }
  return true;
}
