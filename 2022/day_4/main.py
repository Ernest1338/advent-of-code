def part1(input_file):
    answer = 0
    for line in input_file:
        left_split = line.strip().split(',')[0].split('-')
        right_split = line.strip().split(',')[1].split('-')
        if ((int(left_split[0]) <= int(right_split[0]) and int(left_split[1]) >= int(right_split[1]))
            or (int(right_split[0]) <= int(left_split[0]) and int(right_split[1]) >= int(left_split[1]))):
            answer += 1
    return answer

def part2(input_file):
    answer = 0
    for line in input_file:
        left_split = line.strip().split(',')[0].split('-')
        right_split = line.strip().split(',')[1].split('-')
        if (int(left_split[0]) in range(int(right_split[0]), int(right_split[1]) + 1)
            or int(left_split[1]) in range(int(right_split[0]), int(right_split[1]) + 1)
            or int(right_split[0]) in range(int(left_split[0]), int(left_split[1]) + 1)
            or int(right_split[1]) in range(int(left_split[0]), int(left_split[1]) + 1)):
            answer += 1
    return answer

def main():
    part = 2

    with open("input.txt", "r") as input_file:
        if part == 1:
            print("Answer: " + str(part1(input_file)))
        elif part == 2:
            print("Answer: " + str(part2(input_file)))

main()
