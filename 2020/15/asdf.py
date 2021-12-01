inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 2 # 1 or 2
answer = 0

if part == 1:
    numbers = inputfile[0].split(",")
    numbers.reverse()
    for i in range(2020-6):
        numLen = len(numbers)
        currentNumber = numbers[0]
        if currentNumber not in numbers[1:]:
            numbers.insert(0, "0")
        else:
            numbers.insert(0, str(numLen-(numLen-numbers[1:].index(currentNumber)-1)))
    answer = numbers[0]

elif part == 2:
    numbers = inputfile[0].split(",")
    numbers.reverse()
    uniqNumbers = []
    for i in range(30000000-6):
        if i%1000==0:
            print(i)
        numLen = len(numbers)
        currentNumber = numbers[0]
        if currentNumber not in uniqNumbers:
            numbers.insert(0, "0")
            uniqNumbers.append(currentNumber)
        else:
            numbers.insert(0, str(numLen-(numLen-numbers[1:].index(currentNumber)-1)))
    answer = numbers[0]

else:
    print("nope")

print("The final answer is: "+str(answer))