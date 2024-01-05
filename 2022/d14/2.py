import sys
from collections import defaultdict
from copy import copy
from typing import DefaultDict, NoReturn

Rocks = DefaultDict[tuple[int, int], str]
Point = tuple[int, int]
Bounds = tuple[Point, Point]


def make_point_line(p1: Point, p2: Point) -> set[Point]:
    dx = 0
    dy = 0
    dummy = copy(p1)

    # lines are not diagonal
    if p1[0] != p2[0]:
        if p1[0] < p2[0]:
            dx = 1
        else:
            dx = -1
    else:
        if p1[1] < p2[1]:
            dy = 1
        else:
            dy = -1

    point_line: set[Point] = set()
    point_line.add(copy(dummy))

    while dummy != p2:
        dummy = (dummy[0] + dx, dummy[1])
        dummy = (dummy[0], dummy[1] + dy)
        point_line.add(copy(dummy))

    return point_line


def get_point(s: str) -> Point:
    x, y = s.split(",")
    return (int(x), int(y))


def get_rocks(rock_input: list[str]) -> Rocks:
    rocks: Rocks = defaultdict(lambda: ".")

    for line in rock_input:
        points = line.replace(" ", "").split("->")

        for i in range(0, len(points) - 1):
            start = get_point(points[i])
            end = get_point(points[i + 1])
            point_line = make_point_line(start, end)

            for p in point_line:
                rocks[p] = "#"

    return rocks


def get_rock_bounds(rocks: Rocks) -> Bounds:
    min_x, max_x = sys.maxsize, 0
    max_y = 0  # the min_y is always 0

    for rock in rocks:
        min_x = min(min_x, rock[0])
        max_x = max(max_x, rock[0])
        max_y = max(max_y, rock[1])

    return ((min_x, max_x), (0, max_y))


def is_point_in_bounds(p: Point, bounds: Bounds) -> bool:
    return (bounds[0][0] <= p[0] <= bounds[0][1]) and (p[1] < bounds[1][1])


def drop_sand(drop_point: Point, rocks: Rocks, floor_y: int) -> None:
    x, y = drop_point

    while y != floor_y - 1 and rocks[(x, y + 1)] == ".":
        y += 1

    if y == floor_y - 1:
        rocks[(x, y)] = "o"
        return
    elif rocks[(x - 1, y + 1)] == ".":
        drop_sand((x - 1, y + 1), rocks, floor_y)
    elif rocks[(x + 1, y + 1)] == ".":
        drop_sand((x + 1, y + 1), rocks, floor_y)
    else:
        rocks[(x, y)] = "o"
        return


if __name__ == "__main__":
    rock_input = open("in.txt").read().splitlines()
    rocks = get_rocks(rock_input)
    bounds = get_rock_bounds(rocks)
    floor_y = bounds[1][1] + 2
    sand_drop_point = (500, 0)

    i = 0
    while rocks[sand_drop_point] != "o":
        drop_sand(sand_drop_point, rocks, floor_y)
        i += 1
    print(i)
