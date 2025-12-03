"""
FILE: Day 02/Python/sol.py
Author: Alex Jones
Desc: Solution to day 2 problems (3 & 4) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 02/data.txt", "r").read().strip().replace("\r","").split("\n")

ranges = [r.split("-") for r in data[0].split(",")]
p1_total = 0
seen = set()
for id_range in data[0].split(","):
    (start_str, end_str) = id_range.split("-")
    start_val, end_val = int(start_str), int(end_str)
    for part_len in range(len(end_str)//2, 0, -1):
        for target_len in range(len(start_str), len(end_str)+1):
            (num_parts, rem) = divmod(target_len, part_len)
            if rem != 0 or num_parts <= 1:
                continue
            for part in range(10**(part_len-1),10**(part_len)):
                invalid_id = int(str(part) * num_parts)
                if invalid_id < start_val or invalid_id > end_val or invalid_id in seen:
                    continue
                seen.add(invalid_id)
                if num_parts == 2:
                    p1_total += invalid_id

print("Problem 3:", p1_total)
print("Problem 4:", sum(seen))
