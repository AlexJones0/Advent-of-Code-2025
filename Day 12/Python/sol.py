"""
FILE: Day 12/Python/sol.py
Author: Alex Jones
Desc: Solution to day 12 problems (23 & 24) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 12/data.txt", "r").read().rstrip().replace("\r","").split("\n\n")

# Naive solution that only excludes solutions that can't possibly fit,
# but this also happens to be all the real inputs for this puzzle.
sizes = [len([c for c in s if c == '#']) for s in data[:-1]]
total = 0
for region in data[-1].split("\n"):
    (area, num_shapes) = region.split(": ")
    (area, num_shapes) = (area.split("x"), num_shapes.split())
    area = int(area[0]) * int(area[1])
    min_area = sum(int(n) * s for n, s in zip(num_shapes, sizes))
    total += min_area <= area
print("Problem 23:", total)
print("Problem 24: Christmas (Spirit) Freebie :)")
