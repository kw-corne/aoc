import ast

lines = open("dbg.txt").read().split("\n\n")
packets = [line.split("\n") for line in lines]


def is_in_order(p1, p2) -> bool | None:
    if type(p1) == int and type(p2) == int:
        if p1 < p2:
            return True
        elif p2 > p1:
            return False
        elif p1 == p2:
            return None

    if type(p1) == list and type(p2) == int:
        return is_in_order(p1, [p2])
    elif type(p1) == int and type(p2) == list:
        return is_in_order([p1], p2)

    assert type(p1) == list and type(p2) == list

    for v1, v2 in zip(p1, p2):
        in_order = is_in_order(v1, v2)
        if in_order is not None:
            return in_order

    if len(p1) == len(p2):
        return None
    elif len(p1) > len(p2):
        return False
    else:
        return True


total = 0
for i, pair in enumerate(packets):
    if is_in_order(ast.literal_eval(pair[0]), ast.literal_eval(pair[1])):
        print("In order")
        total += i + 1
    else:
        print("Not in order")

    print("#" * 70)

    if i == 2:
        break

print(total)
