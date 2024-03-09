lines = open("in.txt").read().splitlines()
lines = lines[0]
dirs = lines.split(",")

horiz_p = 0
vert_p = 0
rotation = 0  # 0 1 2 3 N E S W


def manhattan(a, b):
    return sum(abs(val1 - val2) for val1, val2 in zip(a, b))


for dir in dirs:
    dir = dir.strip()

    rotation_dir = dir[0]
    amount = int(dir[1:])

    if rotation_dir == "R":
        rotation = (rotation + 1) % 4
    else:
        rotation = (rotation - 1) % 4

    if rotation == 0:
        vert_p += amount
    elif rotation == 1:
        horiz_p += amount
    elif rotation == 2:
        vert_p -= amount
    elif rotation == 3:
        horiz_p -= amount

print(manhattan((0, 0), (horiz_p, vert_p)))
