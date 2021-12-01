inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 2 # 1 or 2
win = False
if part==1:
    while win==False:
        for i in inputfile:
            for a in inputfile:
                if i != a and str(2020-int(i))==a:
                    print("Found solution: "+i+"  "+a+"  :  "+str(int(i)*int(a)))
                    win = True
elif part==2:
    while win==False:
        for i in inputfile:
            for a in inputfile:
                for b in inputfile:
                    if str(2020-int(i)-int(b))==a:
                        print("Found solution: "+i+"  "+a+"  "+b+"  :  "+str(int(i)*int(a)*int(b)))
                        win = True
else:
    print("nope")