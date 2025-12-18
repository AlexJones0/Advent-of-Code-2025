"""
FILE: Day 04/Python/sol.py
Author: Alex Jones
Desc: Solution to day 4 problems (7 & 8) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 04/data.txt", "r").read().strip().replace("\r","").split("\n")
data = {(y + x * 1j): c for y, row in enumerate(data) for x, c in enumerate(row)}
ADJACENT = [(a + b) for a in (-1,0,1) for b in (-1j,0,1j) if a or b]

def remove_rolls(data: dict[complex, str]) -> int:
    removed = []
    for pos, c in data.items():
        ngbs = len([1 for dir_ in ADJACENT if data.get(pos + dir_) == '@'])
        if c == '@' and ngbs < 4:
            removed.append(pos)
    for pos in removed:
        data[pos] = '.'
    return len(removed)

p2 = p1 = remove_rolls(data)
print("Problem 7:", p1)

while p1:
    p1 = remove_rolls(data)
    p2 += p1
print("Problem 8:", p2)
