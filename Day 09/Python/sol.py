"""
FILE: Day 09/Python/sol.py
Author: Alex Jones
Desc: Solution to day 9 problems (17 & 18) for Advent of Code 2025, solved in Python 3.
"""
data = open("Day 09/data.txt", "r").read().rstrip().replace("\r","").split("\n")
data = [list(map(int, row.split(","))) for row in data]

# Compress the tile coordinate space to allow much faster spatial
# reasoning and comparison for the polygon.
def compress_coords(coords: list[int], gap: int) -> tuple[dict[int,int]]:
    unique_coords = sorted(list(set(coords)))
    c_map = {c: i * (1 + gap) for i, c in enumerate(unique_coords)}
    return c_map, {v: k for k, v in c_map.items()}

# Rasterize the edges of the polygon (all tiles in the loop).
# When compressed, we assume that these are sufficiently small
# that a 2D array is more efficient compared to a set.
# We use 0 = empty, 1 = edge, 2 = inside.
def rasterize_edges(dims: tuple[int,int], tiles: list[tuple[int,int]]) -> list[list[int]]:
    grid = [[0] * dims[1] for _ in range(dims[0])]
    for (t1, t2) in zip(tiles, tiles[1:] + [tiles[0]]):
        horizontal = t1[0] == t2[0]
        (d1, d2) = (t1[1], t2[1]) if horizontal else (t1[0], t2[0])
        for d in range(min(d1, d2), max(d1, d2)+1):
            (x, y) = (t1[0], d) if horizontal else (d, t1[1])
            grid[x][y] = 1
    return grid

# Raycast to find a coordinate inside the polgyon. We find when we
# are inside by counting the number of edges encountered so far
# (odd = inside), but because we might still be on an outer edge,
# we check both horizontal and vertical rays to see if we are inside.
def raycast_inside_polygon(grid: list[list[int]]) -> tuple[int,int] | None:
    for x in range(len(grid)):
        n = 0
        for y in range(len(grid[x])):
            n += grid[x][y]
            if not grid[x][y] and n % 2 and sum(grid[rx][y] for rx in range(x)) % 2:
                return (x, y)
    return None


# For small edge-case inputs (e.g. the test input), the maximally-compressed
# coordinate space may leave us unable to find an "inside" coordinate that
# originally existed, due to the edges being compressed together. We can
# solve this by adding a gap of 1 between each compressed coordinate. But
# this 4x multiples the polygon search space, so we always try maximal
# compression first (works for most real inputs) and use the general case
# as a less-optimal fallback.
for gap in (0, 1):
    (x_map, x_rmap) = compress_coords([x for (x, _) in data], gap)
    (y_map, y_rmap) = compress_coords([y for (_, y) in data], gap)
    tiles = [(x_map[x], y_map[y]) for (x, y) in data]

    dims = (max(x for x in x_map.values()) + 1, max(y for y in y_map.values()) + 1)
    grid = rasterize_edges(dims, tiles)
    inside = raycast_inside_polygon(grid)
    if inside is not None:
        break

# Now we know where "inside" is, flood fill to find all coordinates
# inside of the polygon.
def flood_fill(x: int, y: int, grid: list[list[int]]):
    neighbours = lambda x, y: [(x + i, y + j) for i in (-1,0,1) for j in (-1,0,1)]
    queue = set([(x, y)])
    while queue:
        pos = queue.pop()
        grid[pos[0]][pos[1]] = 2
        for ngb in neighbours(*pos):
            if not grid[ngb[0]][ngb[1]]:
                queue.add(ngb)

flood_fill(*inside, grid)

# We can quickly determine if a rectangle is inside the polygon by
# checking that all coordinates on its edges are within the polygon.
def rect_in_polygon(p1: tuple[int,int], p2: tuple[int,int], grid: list[list[int]]) -> bool:
    (x1, x2) = (min(p1[0], p2[0]), max(p1[0], p2[0]))
    for x in range(x1, x2 + 1):
        if not grid[x][p1[1]] or not grid[x][p2[1]]:
            return False
    (y1, y2) = (min(p1[1], p2[1]), max(p1[1], p2[1]))
    for y in range(y1 + 1, y2):
        if not grid[p1[0]][y] or not grid[p2[0]][y]:
            return False
    return True

# Since we work in compressed coords, uncompress them first before we
# calculate the area of the rectangle
def uncompressed_area(p1: tuple[int,int], p2: tuple[int,int], x_map: dict[int,int], y_map: dict[int,int]) -> int:
    p1 = (x_map[p1[0]], y_map[p1[1]])
    p2 = (x_map[p2[0]], y_map[p2[1]])
    return (abs(p1[0] - p2[0]) + 1) * (abs(p1[1] - p2[1]) + 1)

# Part 1 - compute the max rectangle area across all tile pairs:
pairs = [(p1, p2) for i, p1 in enumerate(tiles) for p2 in tiles[:i]]
areas = [uncompressed_area(p1, p2, x_rmap, y_rmap) for p1, p2 in pairs]
print("Problem 17:", max(areas))

# Part 2 - compute the max rectangle area only for tile pairs that form
# polygons that lay entirely within (including borders) the polygon:
areas = [area for area, p in zip(areas, pairs) if rect_in_polygon(*p, grid)]
print("Problem 18:", max(areas))
