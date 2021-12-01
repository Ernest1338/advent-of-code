inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 2 # 1 or 2
answer = 0
if part == 1:
    groupAnswers = ""
    for i in inputfile:
        if i == "":
            groupAnswers = "".join(sorted(set(groupAnswers)))
            answer += len(groupAnswers)
            groupAnswers = ""
        else:
            groupAnswers += i
elif part == 2:
    alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']
    groupAnswers = []
    for i in inputfile:
        variable = 0
        if i == "":
            for a in alphabet:
                addOrNot = True
                for b in groupAnswers:
                    if a not in b:
                        addOrNot = False
                if addOrNot:
                    variable += 1
            answer += variable
            groupAnswers = []
        else:
            groupAnswers.append(i)
else:
    print("nope")
print("The final answer is: "+str(answer))