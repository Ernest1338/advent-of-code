shape_score = {"X": 1, "Y": 2, "Z": 3}
opponent_score = {"A": 1, "B": 2, "C": 3}
outcome_score = {"X": 0, "Y": 3, "Z": 6}


def did_win(split):
    diff = opponent_score[split[0]] - shape_score[split[1]]
    return 3 if diff == 0 else (6 if diff == -1 or split == ["C", "X"] else 0)


def choice_score(split):
    if split[1] == "X":  # loose
        tmp = opponent_score[split[0]] + 2
        return tmp - 3 if tmp > 3 else tmp
    elif split[1] == "Y":  # draw
        return opponent_score[split[0]]
    tmp = opponent_score[split[0]] + 1  # win
    return tmp - 3 if tmp > 3 else tmp


part1, part2 = 0, 0
for line in open("./input.txt").readlines():
    split = line.strip().split()
    part1 += shape_score[split[1]] + did_win(split)
    part2 += choice_score(split) + outcome_score[split[1]]
print(f"Part 1 answer: {part1}\nPart 2 answer: {part2}")
