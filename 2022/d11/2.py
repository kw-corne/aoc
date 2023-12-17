def op(x, y, z, old):
    if x == "old":
        x = old
    if z == "old":
        z = old

    x = int(x)
    z = int(z)

    if y == "+":
        return x + z
    elif y == "*":
        return x * z


monkeys = []

for monke in open("in.txt").read().split("\n\n"):
    ml = monke.splitlines()

    items = [int(x) for x in ml[1].split(":")[1].split(",")]
    ops = ml[2].split("=")[1].split()
    div = int(ml[3].split()[-1])
    tru = int(ml[4].split()[-1])
    fals = int(ml[5].split()[-1])

    monkeys.append([items, ops, div, tru, fals])

modulo = 1
for monke in monkeys:
    modulo *= monke[2]


counts = [0] * len(monkeys)
for _ in range(10000):
    for i, m in enumerate(monkeys):
        while len(m[0]) > 0:
            item = m[0].pop(0)
            counts[i] += 1
            item = op(*m[1], item) % modulo
            if item % m[2] == 0:
                monkeys[m[3]][0].append(item)
            else:
                monkeys[m[4]][0].append(item)

m1, m2 = sorted(counts, reverse=True)[:2]
print(m1 * m2)
