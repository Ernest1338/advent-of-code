inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 1 # 1 or 2
answer = 0
if part == 1:
    pass
elif part == 2:
    pass
else:
    print("nope")
print("The final answer is: "+str(answer))