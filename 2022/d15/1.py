import functools

Point = tuple[int, int]
Sensors = dict[Point, Point]


@functools.cache
def manhattan_distance(p1: Point, p2: Point) -> int:
    return sum(abs(a - b) for a, b in zip(p1, p2))


def get_sensors(lines: list[str]) -> Sensors:
    sensors: Sensors = {}

    for line in lines:
        words = line.split()

        sx = int(words[2].split("=")[1][:-1])
        sy = int(words[3].split("=")[1][:-1])

        bx = int(words[8].split("=")[1][:-1])
        by = int(words[9].split("=")[1])

        sensors[(sx, sy)] = (bx, by)

    return sensors


def no_beacon_in_row(sensors: Sensors, row: int) -> int:
    ranges_on_row: set[Point] = set()

    for sensor, beacon in sensors.items():
        dist_to_beacon = manhattan_distance(sensor, beacon)
        row_diff = abs(row - sensor[1])

        if row_diff > dist_to_beacon:
            continue

        xl = sensor[0] - dist_to_beacon + row_diff
        xr = sensor[0] + dist_to_beacon - row_diff

        ranges_on_row.add((xl, xr))

    total = 0
    checked: set[Point] = set()
    for r in ranges_on_row:
        for i in range(r[0], r[1] + 1):
            point = (i, row)

            if point in checked:
                continue
            checked.add(point)

            if point not in sensors.keys() and point not in sensors.values():
                total += 1

    return total


if __name__ == "__main__":
    lines = open("in.txt").read().splitlines()
    sensors = get_sensors(lines)
    row = 2000000
    print(no_beacon_in_row(sensors, row))
