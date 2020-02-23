def solution():
    nums = {*range(3, 1000, 3), *range(5, 1000, 5)}
    return sum(nums)


print(solution())
