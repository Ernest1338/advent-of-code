inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

# TODO:
# [ ] perentacies level extractor
# [ ] perentacies solves based on level
# [ ] base equation solver

if part == 1:
    for i in inputfile:
        openPerent = []
        closePerent = []
        perentSets = []
        for a in range(len(i)):
            if i[a] == "(":
                openPerent.append(a)
            elif i[a] == ")":
                closePerent.append(a)
        counter = len(openPerent)-1
        for a in range(len(openPerent)):
            closestList = []
            stopVar = False
            for b in range(len(closePerent)):
                #print(openPerent[counter], closePerent[b], openPerent[counter]-closePerent[b])
                if openPerent[counter]-closePerent[b] < 0:
                    closestList.append(abs(openPerent[counter]-closePerent[b]))
                else:
                    closestList.append(999999)
            for b in perentSets:
                for c in b:
                    if c == closePerent[closestList.index(min(closestList))]:
                        stopVar = True
            #print(closestList)
            if stopVar:
                stopVar2 = False
                for b in perentSets:
                    for c in b:
                        if c == closePerent[closestList.index(min(closestList))+1]:
                            stopVar2 = True
                if stopVar2:
                    stopVar3 = False
                    for b in perentSets:
                        for c in b:
                            if c == closePerent[closestList.index(min(closestList))+2]:
                                stopVar3 = True
                    if stopVar3:
                        stopVar4 = False
                        for b in perentSets:
                            for c in b:
                                if c == closePerent[closestList.index(min(closestList))+3]:
                                    stopVar4 = True
                        if stopVar4:
                            perentSets.append([openPerent[counter], closePerent[closestList.index(min(closestList))+4]])
                        else:
                            perentSets.append([openPerent[counter], closePerent[closestList.index(min(closestList))+3]])
                    else:
                        perentSets.append([openPerent[counter], closePerent[closestList.index(min(closestList))+2]])
                else:
                    perentSets.append([openPerent[counter], closePerent[closestList.index(min(closestList))+1]])
            else:
                perentSets.append([openPerent[counter], closePerent[closestList.index(min(closestList))]])
            #print(perentSets)
            counter -= 1
        #print(openPerent)
        #print(closePerent)
        '''
        newEquation = i.replace(" ", "")
        stack = []
        toSolve = []
        print(newEquation)
        newNewEquation = newEquation
        for a in range(len(newEquation)):
            if newEquation[a].isnumeric():
                if len(toSolve)!=0:
                    toSolve.append(newEquation[a])
                    newNewEquation = str(eval("".join(toSolve)))+newNewEquation[a:]
                    toSolve = []
                    print("NEWNEWEQUATION= "+str(newNewEquation))
                else:
                    toSolve.append(newEquation[a])
            elif newEquation[a] == "+" or newEquation[a] == "*":
                toSolve.append(newEquation[a])
            elif newEquation[a] == "(":
                for b in range(len(toSolve)):
                    stack.append(toSolve[b])
                print("STACK: "+str(stack))
                toSolve = []
            else:
                print("BEFORE  "+str(newNewEquation))
                newNewEquation = newNewEquation[:a]+newNewEquation[a+1:]
                print("AFTER  "+str(newNewEquation))
            print(toSolve)
        '''
        levelsP = []
        for a in range(len(perentSets)):
            levelsP.append("#")
        for a in range(len(perentSets)):
            toAppend = False
            similiraties = []
            for b in range(len(perentSets)):
                if perentSets[a]==perentSets[b]:
                    pass
                else:
                    for c in range(perentSets[a][0], perentSets[a][1]):
                        if c+1 == perentSets[b][0] or c+1 == perentSets[b][1]:
                            print("C+1: "+str(c+1)+"PERENTb0: "+str(perentSets[b][0])+"PERENTb1: "+str(perentSets[b][1]))
                            similiraties.append("#")
            if len(similiraties)==2:
                levelsP.append(1)
                if toAppend:
                    levelsP.append(2)
            else:
                toAppend = True
        if toAppend:
            levelsP.append(1)
        print(levelsP)





elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer))