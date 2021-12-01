inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

if part == 1:
    def adjacentSeats(seats, row, column):
        adjacentSeatsList = []
        try:
            if (row-1)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row-1][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row-1)>=0 and (column)>=0:
                adjacentSeatsList.append(seats[row-1][column])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row-1)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row-1][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row+1][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column)>=0:
                adjacentSeatsList.append(seats[row+1][column])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row+1][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        return adjacentSeatsList

    for _ in range(102):
        newInputfile = []
        for i in range(len(inputfile)):
            for a in range(len(inputfile[0])):
                adjacentTest = adjacentSeats(inputfile, i, a)
                if inputfile[i][a]=="L" and "#" not in adjacentTest:
                    newInputfile.append("#")
                elif inputfile[i][a]=="#":
                    nmbrOfOccupied = 0
                    for b in adjacentTest:
                        if b == "#":
                            nmbrOfOccupied += 1
                    if nmbrOfOccupied >= 4:
                        newInputfile.append("L")
                    else:
                        newInputfile.append("#")
                elif inputfile[i][a]=="L" and "#" in adjacentTest:
                    newInputfile.append("L")
                else:
                    newInputfile.append(".")
        newNewInputfile = []
        var = 1
        toAppend = ""
        for i in newInputfile:
            if var < len(inputfile[0]):
                toAppend += str(i)
                var += 1
            else:
                toAppend += str(i)
                newNewInputfile.append(toAppend)
                toAppend = ""
                var = 1
        inputfile = newNewInputfile
    for i in newNewInputfile:
        for a in i:
            if a=="#":
                answer += 1

elif part == 2:
    def adjacentSeats(seats, row, column):
        adjacentSeatsList = []
        try:
            if (row-1)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row-1][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row-1)>=0 and (column)>=0:
                adjacentSeatsList.append(seats[row-1][column])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row-1)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row-1][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column+1)>=0:
                adjacentSeatsList.append(seats[row+1][column+1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column)>=0:
                adjacentSeatsList.append(seats[row+1][column])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row+1)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row+1][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        try:
            if (row)>=0 and (column-1)>=0:
                adjacentSeatsList.append(seats[row][column-1])
            else:
                adjacentSeatsList.append(".")
        except:
            adjacentSeatsList.append(".")
        return adjacentSeatsList

    for _ in range(102):
        newInputfile = []
        for i in range(len(inputfile)):
            for a in range(len(inputfile[0])):
                adjacentTest = adjacentSeats(inputfile, i, a)
                if inputfile[i][a]=="L" and "#" not in adjacentTest:
                    newInputfile.append("#")
                elif inputfile[i][a]=="#":
                    nmbrOfOccupied = 0
                    for b in adjacentTest:
                        if b == "#":
                            nmbrOfOccupied += 1
                    if nmbrOfOccupied >= 4:
                        newInputfile.append("L")
                    else:
                        newInputfile.append("#")
                elif inputfile[i][a]=="L" and "#" in adjacentTest:
                    newInputfile.append("L")
                else:
                    newInputfile.append(".")
        newNewInputfile = []
        var = 1
        toAppend = ""
        for i in newInputfile:
            if var < len(inputfile[0]):
                toAppend += str(i)
                var += 1
            else:
                toAppend += str(i)
                newNewInputfile.append(toAppend)
                toAppend = ""
                var = 1
        inputfile = newNewInputfile
    for i in newNewInputfile:
        for a in i:
            if a=="#":
                answer += 1

else:
    print("nope")

print("The final answer is: "+str(answer))