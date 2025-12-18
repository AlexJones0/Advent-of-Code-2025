"""
FILE: Day 03/Python/sol.py
Author: Alex Jones
Desc: Solution to day 3 problems (5 & 6) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 03/data.txt", "r").read().strip().replace("\r","").split("\n")

def max_joltage(num_batteries: int, bank: str) -> int:
    total = 0
    start, end = 0, len(bank) - num_batteries
    for _ in range(num_batteries):
        idx = max(range(start, end + 1), key=bank.__getitem__)
        total = total * 10 + int(bank[idx])
        start, end = idx + 1, end + 1
    return total

print("Problem 5:", sum(max_joltage(2, bank) for bank in data))
print("Problem 6:", sum(max_joltage(12, bank) for bank in data))
