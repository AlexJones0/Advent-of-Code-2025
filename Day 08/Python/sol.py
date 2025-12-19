"""
FILE: Day 08/Python/sol.py
Author: Alex Jones
Desc: Solution to day 8 problems (15 & 16) for Advent of Code 2025, solved in Python 3.
"""
import heapq

NUM_CONNECTIONS = 1000  # Change to 10 for test input
data = open("Day 08/data.txt", "r").read().rstrip().replace("\r","").split("\n")

nodes = [list(map(int, r.split(","))) for r in data]
dist = lambda a, b: sum((a[i]-b[i])*(a[i]-b[i]) for i in range(3))
pairs = [(dist(nodes[i], nodes[j]), j, i) for i in range(len(nodes)) for j in range(i)]
heapq.heapify(pairs)

circuits = [[i] for i in range(len(nodes))]

def try_make_connection() -> int:
    (_, i, j) = heapq.heappop(pairs)
    i_circuit = [c for c in circuits if i in c][0]
    j_circuit = [c for c in circuits if j in c][0]
    if i_circuit != j_circuit:
        circuits.remove(j_circuit)
        i_circuit.extend(j_circuit)
        if len(circuits) == 1:
            return nodes[i][0] * nodes[j][0]
    return 0

for _ in range(NUM_CONNECTIONS):
    try_make_connection()
lens = sorted([len(c) for c in circuits], reverse=True)[:3]
print("Problem 15:", lens[0] * lens[1] * lens[2])

while (p2 := try_make_connection()) == 0: continue
print("Problem 16:", p2)
