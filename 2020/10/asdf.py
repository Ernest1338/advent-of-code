inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 2 # 1 or 2
answer = 0

if part == 1:
    inputfile.append('0')
    inputfile = [int(x) for x in inputfile]
    inputfile.sort()
    buildIn = max(inputfile)+3
    inputfile.append(buildIn)
    ones = 0
    twos = 0
    threes = 0
    for i in range(len(inputfile)):
        try:
            difference = inputfile[i+1]-inputfile[i]
            if difference == 1:
                ones += 1
            elif difference == 2:
                twos += 1
            elif difference == 3:
                threes += 1
            else:
                print("what?")
        except:
            pass
    answer = ones*threes

elif part == 2:
    inputfile.append('0')
    inputfile = [int(x) for x in inputfile]
    inputfile.sort()
    buildIn = max(inputfile)+3
    inputfile.append(buildIn)
    print(inputfile)
    differences = []
    ones = 0
    twos = 0
    threes = 0
    for i in range(len(inputfile)):
        try:
            difference = inputfile[i+1]-inputfile[i]
            if difference == 1:
                differences.append(1)
                ones += 1
            elif difference == 2:
                differences.append(2)
                twos += 1
            elif difference == 3:
                differences.append(3)
                threes += 1
            else:
                print("what?")
        except:
            pass
    answer = ones*threes
    print(differences)

else:
    print("nope")
print("The final answer is: "+str(answer))