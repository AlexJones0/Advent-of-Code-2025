"""
FILE: Day 01/Python/sol.py
Author: Alex Jones
Desc: Solution to day 1 problems (1 & 2) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 01/data.txt", "r").read().strip().replace("\r", "").split("\n")
turns = [int(t[1:]) * (1 if t[0] == "R" else -1) for t in data]

DIAL_SIZE = 100
dial, zeroes = 50, 0
for turn in turns:
    dial = (dial + turn) % DIAL_SIZE
    zeroes += dial == 0

print("Problem 1:", zeroes)

dial, zeroes = 50, 0
for turn in turns:
    was_zero = dial == 0
    (rots, dial) = divmod(dial + turn, DIAL_SIZE)
    zeroes += abs(rots) + (rots <= 0 and dial == 0) - (rots < 0 and was_zero)

print("Problem 2:", zeroes)
