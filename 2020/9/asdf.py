inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 2 # 1 or 2
answer = 0

if part == 1:
    preamble = 25
    for i in range(preamble, len(inputfile)):
        preambleList = inputfile[int(i)-25:i]
        addingList = []
        for a in range(len(preambleList)):
            for b in range(len(preambleList)):
                if a != b:
                    addingList.append(int(preambleList[a])+int(preambleList[b]))
        if int(inputfile[i]) in addingList:
            pass
        else:
            answer = inputfile[i]
            break

elif part == 2:
    number = 1504371145
    pointer = 0
    size = 17
    times = len(inputfile)-size+1
    for i in range(0, times):
        sumNmbrs = 0
        individuals = []
        for a in range(size):
            sumNmbrs += int(inputfile[pointer+a])
            individuals.append(int(inputfile[pointer+a]))
        if sumNmbrs == number:
            answer = max(individuals) + min(individuals)
        pointer += 1

else:
    print("nope")

print("The final answer is: "+str(answer))