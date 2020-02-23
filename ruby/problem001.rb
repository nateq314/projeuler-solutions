def solution
  threes = (3..999).step(3)
  fives = (5..995).step(5)
  (threes + fives).uniq.sum
end

puts solution