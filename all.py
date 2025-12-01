"""
FILE: all.py
Author: Alex Jones
Desc: A small hacky script to scan for all Python solutions in day directories for Advent of Code 2025 and run the script for each day in order.
Imported from my Advent-of-Code-2023 repository.
"""
import os
from pathlib import Path
from subprocess import call

dirs = [f.path for f in os.scandir(os.getcwd()) if f.is_dir()]
day_dirs = [f for f in dirs if len(f.split(" ")) > 1 and f.split(" ")[-2].endswith("Day")]
day_dirs = sorted(day_dirs, key=lambda fn: int(fn.split("\\")[-1].split(" ")[1]))
for dir_ in day_dirs:
    sol_path = Path(dir_).joinpath("Python/sol.py")
    if os.path.exists(sol_path):
        call(["python3", sol_path])
