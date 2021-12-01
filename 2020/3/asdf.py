inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 2 # 1 or 2
trees = 0
if part == 1:
    cord = 0
    for i in inputfile:
        if i[cord]=="#":
            trees += 1
        if cord+3>=31:
            cord += 3
            cord -= 31
        else:
            cord += 3
elif part == 2:
    slope1 = 0
    slope2 = 240
    slope3 = 0
    slope4 = 0
    slope5 = 0
    cord = 0
    for i in inputfile: # first slope 1-1
        if i[cord]=="#":
            slope1 += 1
        if cord+1>=31:
            cord += 1
            cord -= 31
        else:
            cord += 1
    cord = 0
    for i in inputfile: # third slope 5-1
        if i[cord]=="#":
            slope3 += 1
        if cord+5>=31:
            cord += 5
            cord -= 31
        else:
            cord += 5
    cord = 0
    for i in inputfile: # fourth slope 7-1
        if i[cord]=="#":
            slope4 += 1
        if cord+7>=31:
            cord += 7
            cord -= 31
        else:
            cord += 7
    cord = 0
    counter = 2
    for i in inputfile: # third slope 1-2
        if counter%2==0:
            if i[cord]=="#":
                slope5 += 1
            if cord+1>=31:
                cord += 1
                cord -= 31
            else:
                cord += 1
        else:
            pass
        counter += 1
    trees = slope1*slope2*slope3*slope4*slope5
else:
    print("nope")
print("The final answer is: "+str(trees))