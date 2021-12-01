inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

# Notes: DICTIONARY, edge cases eg: contain no other bags 

part = 2 # 1 or 2
answer = []

if part == 1:
    bagDict = {}
    for i in inputfile:
        lineList = i.split(" ")
        key = ""
        if lineList[4]=="no":
            pass
        else:
            key = ""+str(lineList[0])+" "+str(lineList[1])
            if key in bagDict: 
                bagDict[key].append(str(lineList[5]+" "+lineList[6]))
            else:
                bagDict[key] = []
                bagDict[key].append(str(lineList[5]+" "+lineList[6]))
            try:
                if lineList[8].isnumeric():
                    bagDict[key].append(str(lineList[9]+" "+lineList[10]))
                if lineList[12].isnumeric():
                    bagDict[key].append(str(lineList[13]+" "+lineList[14]))
                if lineList[16].isnumeric():
                    bagDict[key].append(str(lineList[17]+" "+lineList[18]))
                if lineList[20].isnumeric():
                    bagDict[key].append(str(lineList[21]+" "+lineList[22]))
            except:
                pass
    def recurFunc(var):
        inside = []
        for i in bagDict:
            for a in bagDict[i]:
                if a == var:
                    inside.append(i)
        return inside
    toCheck = ['dim salmon', 'plaid blue', 'dark silver', 'clear green', 'shiny brown', 'plaid cyan', 'wavy coral']
    checked = []
    def recurReal():
        global toCheck
        global checked
        global answer
        for _ in range(len(bagDict)):
            toCheck = list(dict.fromkeys(toCheck))
            for i in toCheck:
                if i not in checked:
                    if recurFunc(i) != []:
                        for a in recurFunc(i):
                            toCheck.append(a)
                    checked.append(i)
        answer = toCheck
    answer = toCheck
    recurReal()
    answer = list(dict.fromkeys(answer))

elif part == 2:
    bagDict = {}
    for i in inputfile:
        lineList = i.split(" ")
        key = ""
        if lineList[4]=="no":
            pass
        else:
            key = ""+str(lineList[0])+" "+str(lineList[1])
            if key in bagDict: 
                bagDict[key].append([str(lineList[4]), str(lineList[5]+" "+lineList[6])])
            else:
                bagDict[key] = []
                bagDict[key].append([str(lineList[4]), str(lineList[5]+" "+lineList[6])])
            try:
                if lineList[8].isnumeric():
                    bagDict[key].append([str(lineList[8]), str(lineList[9]+" "+lineList[10])])
                if lineList[12].isnumeric():
                    bagDict[key].append([str(lineList[12]), str(lineList[13]+" "+lineList[14])])
                if lineList[16].isnumeric():
                    bagDict[key].append([str(lineList[16]), str(lineList[17]+" "+lineList[18])])
                if lineList[20].isnumeric():
                    bagDict[key].append([str(lineList[20]), str(lineList[21]+" "+lineList[22])])
            except:
                pass
    #print(bagDict['muted brown'])
    toCheck = [['2', 'mirrored blue'], ['1', 'muted brown'], ['3', 'dim purple']]
    secondList = []
    checked = []
    def recurReal():
        global toCheck
        global secondList
        global checked
        global answer
        for _ in range(len(bagDict)):
            try:
                for i in toCheck:
                    for _ in range(int(i[0])):
                        answer.append("$")
                    secondList.append(i[1])
                for i in secondList:
                    if i not in checked:
                        toCheck.append(bagDict[i])
                        checked.append(i)
            except:
                pass
    recurReal()

    # TOO HARD :(

else:
    print("nope")
print("The final answer is: "+str(len(answer)))