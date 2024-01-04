import sys
from heapq import heappop, heappush


def in_grid(pos: tuple[int, int]) -> bool:
    return (0 <= pos[0] < len(heightmap)) and (0 <= pos[1] < len(heightmap[0]))


def symbol_pos(symbol: str) -> tuple[int, int]:
    for i in range(0, len(heightmap)):
        for j in range(0, len(heightmap[0])):
            if heightmap[i][j] == symbol:
                return (i, j)

    raise ValueError


def all_symbol_pos(symbol: str) -> list[tuple[int, int]]:
    positions = []

    for i in range(0, len(heightmap)):
        for j in range(0, len(heightmap[0])):
            if heightmap[i][j] == symbol:
                positions.append((i, j))

    return positions


def add_tup(t1: tuple[int, int], t2: tuple[int, int]) -> tuple[int, int]:
    res = tuple([sum(x) for x in zip(t1, t2)])
    assert len(res) == 2
    return res


def height(pos: tuple[int, int]) -> int:
    char = heightmap[pos[0]][pos[1]]

    if char == "S":
        char = "a"
    elif char == "E":
        char = "z"

    return ord(char)


heightmap = open("in.txt").read().splitlines()

start_positions = [symbol_pos("S")] + all_symbol_pos("a")
end_pos = symbol_pos("E")
adj = [(0, 1), (0, -1), (1, 0), (-1, 0)]

shortest = sys.maxsize

# TODO: Could optimize by checking which 'a' we already know the distance to
# the end goal of, but this runs in a couple seconds anyway
for start_pos in start_positions:
    seen = set()
    heap = []
    heappush(heap, (0, start_pos))

    while len(heap) > 0:
        cost, pos = heappop(heap)

        if pos == end_pos:
            shortest = min(cost, shortest)
            break

        if pos in seen:
            continue
        seen.add(pos)

        for dir in adj:
            next_pos = add_tup(pos, dir)
            if not in_grid(next_pos):
                continue
            if height(next_pos) > height(pos) + 1:
                continue
            heappush(heap, (cost + 1, next_pos))

print(shortest)
