"""
FILE: Day 05/Python/sol.py
Author: Alex Jones
Desc: Solution to day 5 problems (9 & 10) for Advent of Code 2025, solved in Python 3.
"""
NOT_IMPLEMENTED = "Not Yet Implemented"
data = [item.split("\n") for item in open("Day 05/data.txt", "r").read().strip().replace("\r","").split("\n\n")]
(ranges, ids) = data

ids = [int(id) for id in ids]
ranges = sorted([tuple(map(int, r.split("-"))) for r in ranges])
franges = []
for i in range(1,len(ranges)):
    if ranges[i-1][1] < ranges[i][0]:
        franges.append(ranges[i-1])
    else:
        ranges[i] = (ranges[i-1][0], max(ranges[i-1][1], ranges[i][1]))
franges.append(ranges[-1])

print("Problem 9:", sum(
    any(end >= id_ and start <= id_ for (start, end) in franges)
    for id_ in ids)
)
print("Problem 10:", sum(end - start + 1 for (start, end) in franges))
