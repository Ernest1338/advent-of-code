inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
numberOfGoodPasswords = 0
part = 2 # 1 or 2
if part == 1:
    for i in inputfile:
        bothNumbers = ""
        leftNumber = ""
        rightNumber = ""
        for a in i:
            if a == " ":
                break
            else:
                bothNumbers += a
        for a in bothNumbers:
                if a == "-":
                    break
                else:
                    leftNumber += a
        rightNumber = bothNumbers[len(leftNumber)+1:]
        letter = i[len(bothNumbers)+1]
        password = i[len(bothNumbers)+4:]
        numberOfLetters = 0
        for a in password:
            if a == letter:
                numberOfLetters += 1
        if numberOfLetters >= int(leftNumber) and numberOfLetters <= int(rightNumber):
            numberOfGoodPasswords += 1
elif part == 2:
    for i in inputfile:
        bothNumbers = ""
        leftNumber = ""
        rightNumber = ""
        for a in i:
            if a == " ":
                break
            else:
                bothNumbers += a
        for a in bothNumbers:
                if a == "-":
                    break
                else:
                    leftNumber += a
        rightNumber = bothNumbers[len(leftNumber)+1:]
        letter = i[len(bothNumbers)+1]
        password = i[len(bothNumbers)+4:]
        try:
            if password[int(leftNumber)-1]==letter or password[int(rightNumber)-1]==letter:
                if password[int(leftNumber)-1]==letter and password[int(rightNumber)-1]==letter:
                    pass
                # print("this is a good password, because "+password+"  "+str(leftNumber)+"  "+str(rightNumber)+"  "+letter)
                else:
                    numberOfGoodPasswords += 1
        except:
            print("ERROR")
else:
    print("nope")

print("The final answer is: "+str(numberOfGoodPasswords))
