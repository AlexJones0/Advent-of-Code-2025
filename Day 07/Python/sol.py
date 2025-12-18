"""
FILE: Day 07/Python/sol.py
Author: Alex Jones
Desc: Solution to day 7 problems (13 & 14) for Advent of Code 2025, solved in Python 3.
"""
import functools

data = open("Day 07/data.txt", "r").read().rstrip().replace("\r","").split("\n")

start = (0, data[0].index('S'))
beams = {start}

def advance_step(beams: set[tuple[int,int]]) -> int:
    removed = []
    added = []
    for (y, x) in beams:
        removed.append((y, x))
        if data[y+1][x] == '^':
            added += [(y+1, x-1), (y+1, x+1)]
        else:
            added.append((y+1,x))
    beams -= set(removed)
    beams |= set(added)

    return len(added) - len(removed)


print("Problem 13:", sum(advance_step(beams) for _ in range(len(data) - 1)))

@functools.lru_cache
def num_splits(y, x) -> int:
    if y == len(data) - 1:
        return 1
    if data[y+1][x] != '^':
        return num_splits(y+1, x)
    return num_splits(y+1, x-1) + num_splits(y+1, x+1)

print("Problem 14:", num_splits(*start))
