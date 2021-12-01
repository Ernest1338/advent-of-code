inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

# check if all adresses are under 100k

part = 1 # 1 or 2
answer = 0

if part == 1:
    memory = []
    for _ in range(100000):
        memory.append(0)
    for i in inputfile:
        if i[:4]=="mask":
            mask = i[7:]
        elif i[:3]=="mem":
            address = int(i[4:i.index("]")])
            value = int(i[i.index("=")+2:])
            valueBin = bin(value)[2:]
            while len(valueBin)<36:
                valueBin = "0"+valueBin
            valueToWriteBin = ""
            for i in range(36):
                if mask[i]=="1":
                    valueToWriteBin += "1"
                elif mask[i]=="0":
                    valueToWriteBin += "0"
                elif mask[i]=="X":
                    valueToWriteBin += valueBin[i]
                else:
                    print("What?")
            valueToWrite = int(valueToWriteBin, 2)
            memory[address] = valueToWrite
    for i in memory:
        answer += i

elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer))