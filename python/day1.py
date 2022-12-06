with open('../data/day1.txt') as doc:
    data = doc.read().split('\n\n')

# elf with max calories
max_calories = max(map(lambda x: sum([int(z) for z in x.split('\n') if z]), data))


# top three elves
result = list(map(lambda x: sum([int(z) for z in x.split('\n') if z]), data))
result.sort(reverse=True)
top_three_sum = sum(result[:3])


# results
assert max_calories == 71124
assert top_three_sum == 204639