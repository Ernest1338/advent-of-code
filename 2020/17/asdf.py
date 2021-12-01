inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

if part == 1:
    lenOfAll = len(inputfile[0])+12
    dotsBetAft = "......"
    newStart = []
    for i in range(6):
        newStart.append(dotsBetAft+"."*len(inputfile[0])+dotsBetAft)
    for i in inputfile:
        newStart.append(dotsBetAft+i+dotsBetAft)
    for i in range(6):
        newStart.append(dotsBetAft+"."*len(inputfile[0])+dotsBetAft)
    emptyPane = []
    for i in range(lenOfAll):
        emptyPane.append("."*lenOfAll)
    newStart2 = []
    newStart2.append(emptyPane)
    newStart2.append(newStart)
    newStart2.append(emptyPane)
    counter = 0
    newNewStart2 = []
    for i in newStart2:
        countHashes = 0
        if counter == 0 or counter == len(newStart2)-1:
            if counter == 0:
                for a in range(len(i)):
                    newNewStart2.append([])
                    for b in range(len(i[a])):
                        try:
                            if newStart2[1][a-1][b-1]=="#":
                                countHashes += 1
                            elif newStart2[1][a-1][b]=="#":
                                countHashes += 1
                            elif newStart2[1][a-1][b+1]=="#":
                                countHashes += 1
                            elif newStart2[1][a][b-1]=="#":
                                countHashes += 1
                            elif newStart2[1][a][b+1]=="#":
                                countHashes += 1
                            elif newStart2[1][a+1][b-1]=="#":
                                countHashes += 1
                            elif newStart2[1][a+1][b]=="#":
                                countHashes += 1
                            elif newStart2[1][a+1][b+1]=="#":
                                countHashes += 1
                            if newStart2[1][a][b]=="." and countHashes==3:
                                newNewStart2[a].append("#")
                            else:
                                newNewStart2[a].append(".")
                            if newStart2[1][a][b]=="#" and countHashes==2:
                                newNewStart2[a].append("#")
                            elif newStart2[1][a][b]=="#" and countHashes==3:
                                newNewStart2[a].append("#")
                            else:
                                newNewStart2[a].append(".")
                        except:
                            pass
                for x in newNewStart2:
                    toPrint = ""
                    for y in x:
                        toPrint += y
                    print(toPrint)

            elif counter == len(newStart2)-1:
                for a in range(len(i)):
                    for b in range(len(i[a])):
                        if newStart2[-2][a][b]=="#":
                            countHashes += 1
                        #print(a,b)
        else:
            for a in range(len(i)):
                for b in range(len(i[a])):
                    if newStart2[1][a][b]=="#":
                        countHashes += 1
                    #print(a,b)
        counter += 1


elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer)) 