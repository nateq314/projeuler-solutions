int solution() {
  var threes = new List<int>.generate(999 ~/ 3, (int index) => (index + 1) * 3);
  var fives = new List<int>.generate(999 ~/ 5, (int index) => (index + 1) * 5);
  return (threes + fives).toSet().reduce((sum, value) => sum + value);
}

void main() {
  print(solution());
}