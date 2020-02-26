int solution() {
  var a = 1, b = 2;
  var sum = 0;

  while (b < 4000000) {
    if (b.isEven) sum += b;
    var newFib = a + b;
    a = b;
    b = newFib;
  }

  return sum;
}

void main() {
  print(solution());
}
