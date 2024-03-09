from dataclasses import dataclass


@dataclass
class Valve:
    name: str
    flow: int
    tunnels: list[str]
    is_open: bool = False


def get_valves(valve_input: list[str]) -> dict[str, Valve]:
    valves: dict[str, Valve] = {}

    for line in valve_input:
        words = line.split()

        name = words[1]
        flow = int(words[4].split("=")[1][:-1])
        tunnels = "".join(words[9:]).split(",")

        valves[name] = Valve(name, flow, tunnels)

    return valves


mem = {}


def get_max_pressure(
    curr_valve: Valve,
    minutes_left: int,
    curr_p: int,
    valves: dict[str, Valve],
) -> int:
    if minutes_left <= 1 or all(v.is_open for v in valves.values()):
        return curr_p

    k = (curr_valve, minutes_left, curr_p)
    if k in mem:
        return mem[k]

    def aaa(ml: int, cr_p: int) -> int:
        pressures = []
        for name in curr_valve.tunnels:
            next_valve = valves[name]
            pressures.append(get_max_pressure(next_valve, ml, cr_p, valves))
        return max(pressures)

    move_on = curr_valve.flow == 0 or curr_valve.is_open
    if move_on:
        return aaa(minutes_left - 1, curr_p)
    else:
        return max(
            aaa(minutes_left - 1, curr_p),
            aaa(minutes_left - 2, curr_p + (minutes_left - 1) * curr_valve.flow),
        )


if __name__ == "__main__":
    valve_input = open("dbg.txt").read().splitlines()
    valves = get_valves(valve_input)
    max_pressure = get_max_pressure(valves["AA"], 30, 0, valves)
    print(max_pressure)
