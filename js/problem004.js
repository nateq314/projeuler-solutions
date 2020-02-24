function isPalindromic(n) {
  if (!Number.isInteger(n)) throw new Error("invalid input");
  if (n < 0) return false;
  if (n < 10) return true;
  const digits = String(n).split("");
  const reversed = digits.slice().reverse();
  return digits.every((digit, i) => digit == reversed[i]);
}

function solution() {
  let highest = 0;
  for (let a = 999; a >= 100; a--) {
    if (a * a < highest) break;
    for (let b = a; b >= 100; b--) {
      const product = a * b;
      if (product < highest) break;
      if (!isPalindromic(product)) continue;
      if (product > highest) highest = product;
    }
  }
  return highest;
}

console.log(solution());
