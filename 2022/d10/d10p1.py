x = [1]

for instr in open("in.txt").read().splitlines():
    instr = instr.split()

    if instr[0] == "noop":
        x.append(x[-1])
    elif instr[0] == "addx":
        x.append(x[-1])
        x.append(x[-1] + int(instr[1]))


print(sum(v * x[v - 1] for v in [20, 60, 100, 140, 180, 220]))
