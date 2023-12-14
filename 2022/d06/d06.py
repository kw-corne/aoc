LEN = 14


def marker(buf: str) -> int:
    chars = {}

    for i, c in enumerate(buf):
        if i >= LEN:
            if all(v == 1 for v in chars.values() if v > 0):
                return i

        if c in chars:
            chars[c] += 1
        else:
            chars[c] = 1

        if i >= LEN:
            k = buf[i - LEN]
            chars[k] = max(0, chars[k] - 1)

    raise Exception("Didnt find answer")


for buf in open("in.txt").read().splitlines():
    print(marker(buf))
