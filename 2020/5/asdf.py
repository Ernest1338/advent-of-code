inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 2 # 1 or 2
seatIDList = []
answer = 0
if part == 1:
    for i in inputfile:
        leftBinary = ""
        rightBinary = ""
        seatID = 0
        for a in i:
            if a == "F":
                leftBinary += "0"
            elif a == "B":
                leftBinary += "1"
            elif a == "R":
                rightBinary += "1"
            elif a == "L":
                rightBinary += "0"
            else:
                print("wtf")
        row = int(leftBinary,2)
        column = int(rightBinary,2)
        seatID = row * 8 + column
        seatIDList.append(seatID)
    seatIDList.sort()
    answer = seatIDList[-1]
elif part == 2:
    for i in inputfile:
        leftBinary = ""
        rightBinary = ""
        seatID = 0
        for a in i:
            if a == "F":
                leftBinary += "0"
            elif a == "B":
                leftBinary += "1"
            elif a == "R":
                rightBinary += "1"
            elif a == "L":
                rightBinary += "0"
            else:
                print("wtf")
        row = int(leftBinary,2)
        column = int(rightBinary,2)
        seatID = row * 8 + column
        seatIDList.append(seatID)
    seatIDList.sort()
    for i in range(seatIDList[0], seatIDList[-1]):
        try:
            if seatIDList[i+1] != seatIDList[i]+1:
                answer = seatIDList[i]+1
        except:
            pass
else:
    print("nope")
print("The final answer is: "+str(answer))