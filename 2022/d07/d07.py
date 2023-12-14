import sys


class Dir:
    def __init__(self, name, parent, children):
        self.name = name
        self.parent = parent
        self.children = children


class File:
    def __init__(self, name, size, parent):
        self.name = name
        self.size = size
        self.parent = parent


def make_file_tree(lines: list[str]) -> Dir:
    root = Dir("root", None, [])
    ptr = root
    i = 0

    while i < len(lines):
        if i == 0:
            i += 1
            continue

        s = lines[i].split()

        if s[1] == "ls":
            i += 1
            while i < len(lines) and not lines[i].startswith("$"):
                s2 = lines[i].split()
                if s2[0] == "dir":
                    ptr.children.append(Dir(s2[1], ptr, []))
                else:
                    ptr.children.append(File(s2[1], int(s2[0]), ptr))
                i += 1
            continue

        elif s[1] == "cd":
            if s[2] != "..":
                for child in ptr.children:
                    if isinstance(child, Dir):
                        if child.name == s[2]:
                            ptr = child
            else:
                ptr = ptr.parent

        i += 1

    return root


def solution1(root: Dir):
    MAX_SIZE = 100000
    result = {}

    def h(dir) -> int:
        size = 0

        for child in dir.children:
            if isinstance(child, Dir):
                child_size = h(child)

                if child_size <= MAX_SIZE:
                    if child not in result:
                        result[child] = child_size

                size += child_size
            else:
                size += child.size

        return size

    h(root)

    return sum(result.values())


def solution2(root) -> int:
    TARGET_SIZE = 30000000
    TOTAL_SPACE = 70000000
    cache = {}

    def h(dir) -> int:
        size = 0

        for child in dir.children:
            if isinstance(child, Dir):
                child_size = h(child)
                if child not in cache:
                    cache[child] = child_size

                size += child_size
            else:
                size += child.size

        return size

    unused_space = TOTAL_SPACE - h(root)
    to_be_freed = TARGET_SIZE - unused_space

    best = (None, sys.maxsize)
    for k, v in cache.items():
        if v > to_be_freed and v < best[1]:
            best = (k, v)

    return best[1]


def dump(root, depth=0):
    print("  " * depth, "- " + root.name)

    if isinstance(root, Dir):
        for child in root.children:
            dump(child, depth + 1)


lines = open("in.txt").read().splitlines()
root = make_file_tree(lines)
# dump(root)
print("Part 1: ", solution1(root))
print("Part 2: ", solution2(root))
