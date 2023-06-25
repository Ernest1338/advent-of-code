with open("./input.txt") as f:
    sum = 0
    calories = []
    for line in f.readlines():
        line = line.strip()
        if not line:
            calories.append(sum)
            sum = 0
        else:
            sum += int(line)

calories.sort(reverse=True)

print("part 1: ", calories[0])
print("part 2: ", calories[0] + calories[1] + calories[2])
