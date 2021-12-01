inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

if part == 1:
    X = 0
    Z = 0
    direction = 0
    for i in inputfile:
        instruction = i[0]
        instNmbr = i[1:]
        if instruction=="N" or instruction=="E" or instruction=="S" or instruction=="W":
            #print("NESW GROUP: "+instruction)
            if instruction=="N":
                X += int(instNmbr)
            elif instruction=="E":
                Z += int(instNmbr)
            elif instruction=="S":
                X -= int(instNmbr)
            elif instruction=="W":
                Z -= int(instNmbr)
            else:
                print("What? NESW")
        elif instruction=="L" or instruction=="R":
            #print("LR GROUP: "+instruction)
            if instruction=="L":
                direction -= int(instNmbr)
                if direction>360:
                    direction -= 360
                elif direction<0:
                    direction += 360
                else:
                    pass
            elif instruction=="R":
                direction += int(instNmbr)
                if direction>=360:
                    direction -= 360
                elif direction<0:
                    direction += 360
                else:
                    pass
            else:
                print("What? LR")
        elif instruction=="F":
            #print("F GROUP: "+instruction)
            if direction==0:
                Z += int(instNmbr)
            elif direction==90:
                X -= int(instNmbr)
            elif direction==180:
                Z -= int(instNmbr)
            elif direction==270:
                X += int(instNmbr)
            else:
                print("What? F")
        else:
            print("What? ALL")
    X = abs(X)
    Z = abs(Z)
    answer = X+Z

elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer))