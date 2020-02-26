import 'dart:math';
import './isPrime.dart';

List<int> solution(int n) {
  var root = sqrt(n).floor();
  List<int> factors = [];
  var current = n;
  if (current % 3 == 0) {
    factors.add(3);
    current ~/= 3;
  }
  var maybePrime = 5;
  while (maybePrime <= root) {
    if (isPrime(maybePrime) && current % maybePrime == 0) {
      factors.add(maybePrime);
      current ~/= maybePrime;
    }
    maybePrime += 2;
    if (isPrime(maybePrime) && current % maybePrime == 0) {
      factors.add(maybePrime);
      current ~/= maybePrime;
    }
    maybePrime += 4;
  }
  return factors;
}

void main() {
  const n = 600851475143;
  var factors = solution(n);
  print(factors);
  print(factors.last);
}