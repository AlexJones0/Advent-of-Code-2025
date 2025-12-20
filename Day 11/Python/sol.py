"""
FILE: Day 11/Python/sol.py
Author: Alex Jones
Desc: Solution to day 11 problems (21 & 22) for Advent of Code 2025, solved in Python 3.
"""
import functools

data = open("Day 11/data.txt", "r").read().rstrip().replace("\r","").split("\n")
data = [row.split(": ") for row in data]
graph = {row[0]: row[1].split() for row in data}

@functools.lru_cache
def paths(start: str, end: str) -> int:
    if start == end:
        return 1
    return sum(paths(ngb, end) for ngb in graph.get(start, []))

print("Problem 21:", paths("you", "out"))

# Graph is a DAG (acyclic), so either we have `fft` -> `dac` or vice versa
n = paths("fft", "dac")
(fst, snd, n) = ("fft", "dac", n) if n else ("dac", "fft", paths("dac", "fft"))
print("Problem 22:", paths("svr", fst) * n * paths(snd, "out"))
