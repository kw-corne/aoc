import sys


def column(grid, i):
    return [row[i] for row in grid]


def is_tree_visible(tree_grid, i, j) -> bool:
    max_h = tree_grid[i][j]
    n_rows = len(tree_grid)
    n_cols = len(tree_grid[0])

    if all(tree_grid[k][j] < max_h for k in range(i)):
        return True

    if all(tree_grid[k][j] < max_h for k in range(i + 1, n_rows)):
        return True

    if all(tree_grid[i][k] < max_h for k in range(j)):
        return True

    if all(tree_grid[i][k] < max_h for k in range(j + 1, n_cols)):
        return True

    return False


def n_visible(tree_grid):
    visible = len(tree_grid[0]) * 2 + len(tree_grid) * 2 - 4

    for i in range(1, len(tree_grid) - 1):
        for j in range(1, len(tree_grid[i]) - 1):
            if is_tree_visible(tree_grid, i, j):
                visible += 1

    return visible


def scenic_score(tree_grid, i, j):
    sum_score = 1
    max_h = tree_grid[i][j]
    rows = len(tree_grid)
    cols = len(tree_grid[0])

    score = 0
    for k in range(i - 1, -1, -1):
        score += 1
        if tree_grid[k][j] >= max_h:
            break
    sum_score *= score

    score = 0
    for k in range(i + 1, rows):
        score += 1
        if tree_grid[k][j] >= max_h:
            break
    sum_score *= score

    score = 0
    for k in range(j - 1, -1, -1):
        score += 1
        if tree_grid[i][k] >= max_h:
            break
    sum_score *= score

    score = 0
    for k in range(j + 1, cols):
        score += 1
        if tree_grid[i][k] >= max_h:
            break
    sum_score *= score

    return sum_score


def max_scenic_score(tree_grid):
    s = 0
    for i in range(1, len(tree_grid) - 1):
        for j in range(1, len(tree_grid[i]) - 1):
            s = max(s, scenic_score(tree_grid, i, j))

    return s


lines = open("in.txt").read().splitlines()
trees = [list(line) for line in lines]
print("Part 1:", n_visible(trees))
print("Part 2:", max_scenic_score(trees))
