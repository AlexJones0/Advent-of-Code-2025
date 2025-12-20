"""
FILE: Day 10/Python/sol.py
Author: Alex Jones
Desc: Solution to day 10 problems (19 & 20) for Advent of Code 2025, solved in Python 3.
"""
import heapq
import z3

data = open("Day 10/data.txt", "r").read().rstrip().replace("\r","").split("\n")

def min_indicator_presses(machine: str) -> int | None:
    parts = machine.split()
    goal = int(parts[0][-2:0:-1].replace(".", "0").replace("#", "1"), 2)
    if goal == 0:
        return 0
    buttons = [tuple(int(i) for i in b[1:-1].split(",")) for b in parts[1:-1]]
    initial = 0
    seen = set([initial])
    queue = [(0, initial)]
    while queue:
        (length, state) = heapq.heappop(queue)
        for button in buttons:
            next_state = state
            for pos in button:
                next_state ^= (1 << pos)
            if next_state in seen:
                continue
            seen.add(next_state)
            if next_state == goal:
                return length + 1
            heapq.heappush(queue, (length + 1, next_state))
    return None

print("Problem 19:", sum(min_indicator_presses(m) for m in data))

def min_counter_presses(machine: str) -> int | None:
    parts = machine.split()
    buttons = [tuple(int(i) for i in b[1:-1].split(",")) for b in parts[1:-1]]
    goal = tuple(int(c) for c in parts[-1][1:-1].split(","))
    if all(c == 0 for c in goal):
        return 0

    ### Just keep it simple and solve with Z3
    # Create integer symbols for each button
    button_symbols = [z3.Int(f"b_{i}") for i in range(len(buttons))]
    optimizer = z3.Optimize()
    # Buttons must be pressed a non-negative number of times
    optimizer.add([b >= 0 for b in button_symbols])
    # For all buttons that influence a counter, the sum of the presses for
    # each button should equal that counter (for each counter)
    for i, count in enumerate(goal):
        symbols = [sym for b, sym in zip(buttons, button_symbols) if i in b]
        optimizer.add(z3.Sum(*symbols) == count)
    # Our goal is to minimize the total button presses
    optimizer.minimize(z3.Sum(button_symbols))
    # Solve with a SAT solver
    assert optimizer.check() == z3.sat
    model = optimizer.model()
    return sum(model[b].as_long() for b in button_symbols)

print("Problem 20:", sum(min_counter_presses(m) for m in data))
