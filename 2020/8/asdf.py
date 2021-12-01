inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 2 # 1 or 2
answer = 0

if part == 1:
    acc = 0
    pointer = 0
    run = True
    runned = []
    while run:
        try:
            instruction = inputfile[pointer].split(" ")
        except:
            run = False
        if pointer not in runned:
            if instruction[0]=="acc":
                acc += int(instruction[1])
            elif instruction[0]=="nop":
                pass
            elif instruction[0]=="jmp":
                toJump = int(instruction[1])-1
                pointer += toJump
        else:
            run = False
        runned.append(pointer)
        pointer += 1
    answer = acc

elif part == 2:
    acc = 0
    pointer = 0
    run = True
    runned = []

    def loopOrNot(instructions):
        acc = 0
        pointer = 0
        run = True
        runned = []
        loop = False
        while run:
            try:
                instruction = inputfile[pointer].split(" ")
            except:
                run = False
            if instruction[0]=="acc":
                acc += int(instruction[1])
            elif instruction[0]=="nop":
                pass
            elif instruction[0]=="jmp":
                toJump = int(instruction[1])-1
                pointer += toJump
            runned.append(pointer)
            if len(runned)>=10000:
                loop = True
                run = False
            pointer += 1
        return loop # True or False

    for i in range(len(inputfile)):
        if inputfile[i][:3]=="nop":
            if not loopOrNot(inputfile):
                break
            inputfile[i] = "jmp" + str(inputfile[i][3:])
            if not loopOrNot(inputfile):
                break
            inputfile[i] = "nop" + str(inputfile[i][3:])
        elif inputfile[i][:3]=="jmp":
            if not loopOrNot(inputfile):
                break
            inputfile[i] = "nop" + str(inputfile[i][3:])
            if not loopOrNot(inputfile):
                break
            inputfile[i] = "jmp" + str(inputfile[i][3:])

    while run:
        try:
            instruction = inputfile[pointer].split(" ")
        except:
            run = False
        if pointer not in runned:
            if instruction[0]=="acc":
                acc += int(instruction[1])
            elif instruction[0]=="nop":
                pass
            elif instruction[0]=="jmp":
                toJump = int(instruction[1])-1
                pointer += toJump
        else:
            run = False
        runned.append(pointer)
        pointer += 1
    answer = acc

else:
    print("nope")

print("The final answer is: "+str(answer))