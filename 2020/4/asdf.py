inputfile = open('input.txt','r').readlines()
for i in range(len(inputfile)):
    inputfile[i] = inputfile[i].strip('\n')
part = 2 # 1 or 2
answer = 0
if part == 1:
    passportString = ""
    passport = []
    passportEntries = []
    for i in inputfile:
        if i != "":
            passportString += i+" "
        else:
            passport = passportString.split(" ")
            passport = passport[:-1]
            for a in passport:
                passportEntries.append(a[:3])
            if len(passportEntries)==8 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries and "cid" in passportEntries:
                answer += 1
            elif len(passportEntries)==7 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries:
                answer += 1
            passportString = ""  # at the end
            passport = []        # at the end
            passportEntries = [] # at the end
    passport = passportString.split(" ")
    passport = passport[:-1]
    for a in passport:
        passportEntries.append(a[:3])
    if len(passportEntries)==8 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries and "cid" in passportEntries:
        answer += 1
    elif len(passportEntries)==7 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries:
        answer += 1
    
            
elif part == 2:
    passportString = ""
    passport = []
    passportEntries = []
    passportEntriesValues = []
    for i in inputfile:
        if i != "":
            passportString += i+" "
        else:
            passport = passportString.split(" ")
            passport = passport[:-1]
            hclVar = ""
            for a in passport:
                passportEntries.append(a[:3])
                passportEntriesValues.append(a[4:])
            if len(passportEntries)==8 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries and "cid" in passportEntries:
                if int(passportEntriesValues[passportEntries.index("byr")])>=1920 and int(passportEntriesValues[passportEntries.index("byr")])<=2002:
                    if int(passportEntriesValues[passportEntries.index("iyr")])>=2010 and int(passportEntriesValues[passportEntries.index("iyr")])<=2020:
                        if int(passportEntriesValues[passportEntries.index("eyr")])>=2020 and int(passportEntriesValues[passportEntries.index("eyr")])<=2030:
                            if passportEntriesValues[passportEntries.index("hgt")][-2:]=="cm":
                                if int(passportEntriesValues[passportEntries.index("hgt")][:-2])>=150 and int(passportEntriesValues[passportEntries.index("hgt")][:-2])<=193:
                                    hclVar = passportEntriesValues[passportEntries.index("hcl")][1:]
                                    dontContinue = False
                                    if len(hclVar)==6:
                                        for a in hclVar:
                                            if a.isnumeric() or a == "a" or a == "b" or a == "c" or a == "d" or a == "e" or a == "f":
                                                pass
                                            else:
                                                dontContinue = True
                                        if not dontContinue:
                                            if passportEntriesValues[passportEntries.index("ecl")]=="amb" or passportEntriesValues[passportEntries.index("ecl")]=="blu" or passportEntriesValues[passportEntries.index("ecl")]=="brn" or passportEntriesValues[passportEntries.index("ecl")]=="gry" or passportEntriesValues[passportEntries.index("ecl")]=="grn" or passportEntriesValues[passportEntries.index("ecl")]=="hzl" or passportEntriesValues[passportEntries.index("ecl")]=="oth":
                                                if len(passportEntriesValues[passportEntries.index("pid")])==9:
                                                    #print("good, because: "+str(passportEntriesValues[passportEntries.index("pid")]))
                                                    answer += 1
                            elif passportEntriesValues[passportEntries.index("hgt")][-2:]=="in":
                                if int(passportEntriesValues[passportEntries.index("hgt")][:-2])>=59 and int(passportEntriesValues[passportEntries.index("hgt")][:-2])<=76:
                                    hclVar = passportEntriesValues[passportEntries.index("hcl")][1:]
                                    dontContinue = False
                                    if len(hclVar)==6:
                                        for a in hclVar:
                                            if a.isnumeric() or a == "a" or a == "b" or a == "c" or a == "d" or a == "e" or a == "f":
                                                pass
                                            else:
                                                dontContinue = True
                                        if not dontContinue:
                                            if passportEntriesValues[passportEntries.index("ecl")]=="amb" or passportEntriesValues[passportEntries.index("ecl")]=="blu" or passportEntriesValues[passportEntries.index("ecl")]=="brn" or passportEntriesValues[passportEntries.index("ecl")]=="gry" or passportEntriesValues[passportEntries.index("ecl")]=="grn" or passportEntriesValues[passportEntries.index("ecl")]=="hzl" or passportEntriesValues[passportEntries.index("ecl")]=="oth":
                                                if len(passportEntriesValues[passportEntries.index("pid")])==9:
                                                    #print("good, because: "+str(passportEntriesValues[passportEntries.index("pid")]))
                                                    answer += 1
            elif len(passportEntries)==7 and "byr" in passportEntries and "iyr" in passportEntries and "eyr" in passportEntries and "hgt" in passportEntries and "hcl" in passportEntries and "ecl" in passportEntries and "pid" in passportEntries:
                if int(passportEntriesValues[passportEntries.index("byr")])>=1920 and int(passportEntriesValues[passportEntries.index("byr")])<=2002:
                    if int(passportEntriesValues[passportEntries.index("iyr")])>=2010 and int(passportEntriesValues[passportEntries.index("iyr")])<=2020:
                        if int(passportEntriesValues[passportEntries.index("eyr")])>=2020 and int(passportEntriesValues[passportEntries.index("eyr")])<=2030:
                            if passportEntriesValues[passportEntries.index("hgt")][-2:]=="cm":
                                if int(passportEntriesValues[passportEntries.index("hgt")][:-2])>=150 and int(passportEntriesValues[passportEntries.index("hgt")][:-2])<=193:
                                    hclVar = passportEntriesValues[passportEntries.index("hcl")][1:]
                                    dontContinue = False
                                    if len(hclVar)==6:
                                        for a in hclVar:
                                            if a.isnumeric() or a == "a" or a == "b" or a == "c" or a == "d" or a == "e" or a == "f":
                                                pass
                                            else:
                                                dontContinue = True
                                        if not dontContinue:
                                            if passportEntriesValues[passportEntries.index("ecl")]=="amb" or passportEntriesValues[passportEntries.index("ecl")]=="blu" or passportEntriesValues[passportEntries.index("ecl")]=="brn" or passportEntriesValues[passportEntries.index("ecl")]=="gry" or passportEntriesValues[passportEntries.index("ecl")]=="grn" or passportEntriesValues[passportEntries.index("ecl")]=="hzl" or passportEntriesValues[passportEntries.index("ecl")]=="oth":
                                                if len(passportEntriesValues[passportEntries.index("pid")])==9:
                                                    #print("good, because: "+str(passportEntriesValues[passportEntries.index("pid")]))
                                                    answer += 1
                            elif passportEntriesValues[passportEntries.index("hgt")][-2:]=="in":
                                if int(passportEntriesValues[passportEntries.index("hgt")][:-2])>=59 and int(passportEntriesValues[passportEntries.index("hgt")][:-2])<=76:
                                    hclVar = passportEntriesValues[passportEntries.index("hcl")][1:]
                                    dontContinue = False
                                    if len(hclVar)==6:
                                        for a in hclVar:
                                            if a.isnumeric() or a == "a" or a == "b" or a == "c" or a == "d" or a == "e" or a == "f":
                                                pass
                                            else:
                                                dontContinue = True
                                        if not dontContinue:
                                            if passportEntriesValues[passportEntries.index("ecl")]=="amb" or passportEntriesValues[passportEntries.index("ecl")]=="blu" or passportEntriesValues[passportEntries.index("ecl")]=="brn" or passportEntriesValues[passportEntries.index("ecl")]=="gry" or passportEntriesValues[passportEntries.index("ecl")]=="grn" or passportEntriesValues[passportEntries.index("ecl")]=="hzl" or passportEntriesValues[passportEntries.index("ecl")]=="oth":
                                                if len(passportEntriesValues[passportEntries.index("pid")])==9:
                                                    #print("good, because: "+str(passportEntriesValues[passportEntries.index("pid")]))
                                                    answer += 1
            passportString = ""        # at the end
            passport = []              # at the end
            passportEntries = []       # at the end
            passportEntriesValues = [] # at the end
    answer += 1
else:
    print("nope")
print("The final answer is: "+str(answer))