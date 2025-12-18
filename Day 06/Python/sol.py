"""
FILE: Day 06/Python/sol.py
Author: Alex Jones
Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 06/data.txt", "r").read().rstrip("\r\n").replace("\r","").split("\n")

# Parse the max operand length for each operation from the operation row
lengths, length = [], 0
for char in data[-1]:
    if char != " " and length != 0:
        lengths.append(length - 1)
        length = 1
        continue
    length += 1
lengths.append(length)

# Based on these operation lengths, parse the operands/operators from each row
items = [[] for row in data]
for i, row in enumerate(data):
    idx = 0
    for length in lengths:
        items[i].append(row[idx:idx+length])
        idx += length+1

# Transpose the list of operations to account for vertical layout
items = list(map(list, zip(*items)))

# Just use unsafe eval to solve the problem :)
print("Problem 11:", sum(eval(p[-1].join(p[:-1])) for p in items))

# Additional transpose per-operation to account for vertical
# operand layout (per-column). Then again do unsafe eval :)
total = 0
for item in items:
    operands = list([''.join(e).strip() for e in zip(*item[:-1])])
    operator = item[-1].strip()
    total += eval(operator.join(operands[::-1]))
print("Problem 12:", total)
