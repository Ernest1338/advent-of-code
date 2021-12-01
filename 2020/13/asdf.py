inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')

part = 1 # 1 or 2
answer = 0

if part == 1:
    number = int(inputfile[0])
    buses = inputfile[1].split(",")
    buses = [i for i in buses if i != "x"]
    bus23 = ["."]
    bus41 = ["."]
    bus37 = ["."]
    bus479 = ["."]
    bus13 = ["."]
    bus17 = ["."]
    bus29 = ["."]
    bus373 = ["."]
    bus19 = ["."]
    for i in range(1, number+1000):
        if i%int(buses[0])==0:
            bus23.append("D")
        else:
            bus23.append(".")
        if i%int(buses[1])==0:
            bus41.append("D")
        else:
            bus41.append(".")
        if i%int(buses[2])==0:
            bus37.append("D")
        else:
            bus37.append(".")
        if i%int(buses[3])==0:
            bus479.append("D")
        else:
            bus479.append(".")
        if i%int(buses[4])==0:
            bus13.append("D")
        else:
            bus13.append(".")
        if i%int(buses[5])==0:
            bus17.append("D")
        else:
            bus17.append(".")
        if i%int(buses[6])==0:
            bus29.append("D")
        else:
            bus29.append(".")
        if i%int(buses[7])==0:
            bus373.append("D")
        else:
            bus373.append(".")
        if i%int(buses[8])==0:
            bus19.append("D")
        else:
            bus19.append(".")
    firstBuses = []
    firstBuses.append(bus23[number:].index("D"))
    firstBuses.append(bus41[number:].index("D"))
    firstBuses.append(bus37[number:].index("D"))
    firstBuses.append(bus479[number:].index("D"))
    firstBuses.append(bus13[number:].index("D"))
    firstBuses.append(bus17[number:].index("D"))
    firstBuses.append(bus29[number:].index("D"))
    firstBuses.append(bus373[number:].index("D"))
    firstBuses.append(bus19[number:].index("D"))
    answer = int(min(firstBuses))*int(buses[firstBuses.index(min(firstBuses))])

elif part == 2:
    pass

else:
    print("nope")

print("The final answer is: "+str(answer))
