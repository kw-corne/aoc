x = [1]

for instr in open("in.txt").read().splitlines():
    instr = instr.split()

    if instr[0] == "noop":
        x.append(x[-1])
    elif instr[0] == "addx":
        x.append(x[-1])
        x.append(x[-1] + int(instr[1]))


p = ""
for i, v in enumerate(x):
    if v - 1 <= i % 40 <= v + 1:
        p += "#"
    else:
        p += "."

for i in range(6):
    r = i * 40
    r2 = (i + 1) * 40
    print(p[r:r2])
