from copy import copy


def part1():
    visited: set[tuple[int, int]] = set()
    h = (0, 0)
    t = (0, 0)

    visited.add(t)

    for line in open("in.txt").read().splitlines():
        d, s = line.split()

        for _ in range(int(s)):
            if d == "R":
                h = (h[0] + 1, h[1])
            elif d == "L":
                h = (h[0] - 1, h[1])
            elif d == "U":
                h = (h[0], h[1] + 1)
            elif d == "D":
                h = (h[0], h[1] - 1)

            dx = abs(h[0] - t[0])
            dy = abs(h[1] - t[1])

            if dx <= 1 and dy <= 1:
                pass
            else:
                if d == "R":
                    t = (h[0] - 1, h[1])
                elif d == "L":
                    t = (h[0] + 1, h[1])
                elif d == "U":
                    t = (h[0], h[1] - 1)
                elif d == "D":
                    t = (h[0], h[1] + 1)

            visited.add(t)

    print(len(visited))


def dump(parts):
    GZ = 10
    grid = [["." for _ in range(GZ)] for _ in range(GZ)]

    for i, part in enumerate(parts):
        # print(i, part)
        x = part[0] + GZ // 2
        y = GZ // 2 - part[1]
        print(x, y)
        grid[y][x] = i

    for r in grid:
        for c in r:
            print(c, end="")
        print()


def part2():
    visited = set()
    parts = [[0, 0] for _ in range(10)]

    visited.add(tuple(parts[0]))

    for line in open("dbg2.txt").read().splitlines():
        d, s = line.split()

        for _ in range(int(s)):
            if d == "R":
                parts[0][0] += 1
            elif d == "L":
                parts[0][0] -= 1
            elif d == "U":
                parts[0][1] += 1
            elif d == "D":
                parts[0][1] -= 1

            for i in range(len(parts) - 1):
                h = parts[i]
                t = parts[i + 1]

                dx = abs(h[0] - t[0])
                dy = abs(h[1] - t[1])

                if dx <= 1 and dy <= 1:
                    pass
                else:
                    if d == "R":
                        parts[i + 1] = [parts[i][0] - 1, parts[i][1]]
                    elif d == "L":
                        parts[i + 1] = [parts[i][0] + 1, parts[i][1]]
                    elif d == "U":
                        parts[i + 1] = [parts[i][0], parts[i][1] - 1]
                    elif d == "D":
                        parts[i + 1] = [parts[i][0], parts[i][1] + 1]

            dump(parts)
            print("=" * 80)
            visited.add(tuple(parts[-1]))

    print(len(visited))


# part1()
part2()
