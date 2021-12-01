inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

if part == 1:
    rules = []
    for i in inputfile:
        if i != "":
            rules.append(i)
        else:
            break
    
    nearbyTickets = []
    for i in range(len(rules)+5, len(inputfile)):
        nearbyTickets.append(inputfile[i])

    goodNumbers = []

    for i in rules:
        left = False
        right = False
        colonSeen = False
        leftNumbers = ""
        rightNumbers = ""
        for a in i:
            if a == ":":
                colonSeen = True
            if left:
                leftNumbers += a
            elif right:
                rightNumbers += a
            if a == " ":
                if colonSeen:
                    if len(leftNumbers)!=0:
                        left = False
                        right = True
                    else:
                        left = True
        leftNumbers = leftNumbers[:-1]
        rightNumbers = rightNumbers[3:]

        leftNumbers = leftNumbers.split("-")
        rightNumbers = rightNumbers.split("-")

        for a in range(int(leftNumbers[0]), int(leftNumbers[1])+1):
            goodNumbers.append(a)
        for a in range(int(rightNumbers[0]), int(rightNumbers[1])+1):
            goodNumbers.append(a)
        goodNumbers = list(dict.fromkeys(goodNumbers))
    for i in nearbyTickets:
        currentTicket = i.split(",")
        for a in currentTicket:
            if int(a) in goodNumbers:
                pass
            else:
                answer += int(a)
    
elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer))