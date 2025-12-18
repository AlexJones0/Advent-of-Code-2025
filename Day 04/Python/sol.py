"""
FILE: Day 04/Python/sol.py
Author: Alex Jones
Desc: Solution to day 4 problems (7 & 8) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 04/data.txt", "r").read().strip().replace("\r","").split("\n")
data = {(y + x * 1j): c for y, row in enumerate(data) for x, c in enumerate(row)}
ADJ = [(a + b) for a in (-1,0,1) for b in (-1j,0,1j) if a or b]

def remove_rolls(data: dict[complex, str]) -> int:
    return len([
        pos
        for pos, c in data.items()
        if c == '@' and len([d for d in ADJ if data.get(pos + d) == '@']) < 4
    ])

def remove_all_rolls(data: dict[complex, str]) -> int:
    removed = 0
    queue = set(pos for pos, c in data.items() if c == '@')
    while queue:
        pos = queue.pop()
        neighbours = [(pos + d) for d in ADJ if data.get(pos + d) == '@']
        if len(neighbours) < 4:
            removed += 1
            data[pos] = '.'
            queue |= set(neighbours)
    return removed

print("Problem 7:", remove_rolls(data))
print("Problem 8:", remove_all_rolls(data))
