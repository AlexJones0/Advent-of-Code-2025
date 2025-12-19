/*
 * FILE: Day 09/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 9 problems (17 & 18) for Advent of Code 2025, solved in Rust.
 */
#![allow(clippy::needless_range_loop)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use itertools::Itertools;

const ADJACENT_DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbours(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    ADJACENT_DIRS
        .into_iter()
        .map(move |(dx, dy)| ((pos.0 as i32 + dy) as usize, (pos.1 as i32 + dx) as usize))
}

type CoordMap = BTreeMap<usize, usize>;
type CoordRMap = BTreeMap<usize, usize>;

/// Compress the tile coordinate space to allow much faster spatial
/// reasoning and comparison for the polygon.
fn compress_coords(coords: &[usize], gap: usize) -> (CoordMap, CoordRMap) {
    let unique_coords: Vec<usize> = coords
        .iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .sorted()
        .cloned()
        .collect();
    let coord_map = CoordMap::from_iter(
        unique_coords
            .into_iter()
            .enumerate()
            .map(|t| (t.1, t.0 * (1 + gap))),
    );
    let coord_rmap = CoordRMap::from_iter(coord_map.iter().map(|t| (*t.1, *t.0)));
    (coord_map, coord_rmap)
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum GridTile {
    Outside = 0,
    Edge = 1,
    Inside = 2,
}

type Grid = Vec<Vec<GridTile>>;

/// Rasterize the edges of the polygon (all tiles in the loop).
/// When compressed, we assume that these are sufficiently small
/// that a 2D array is more efficient compared to a set.
/// We use 0 = empty
fn rasterize_edges(dimensions: (usize, usize), tiles: &[(usize, usize)]) -> Grid {
    let mut grid =
        Grid::from_iter((0..dimensions.0).map(|_| vec![GridTile::Outside; dimensions.1]));
    let iter = tiles
        .windows(2)
        .map(|w| (&w[0], &w[1]))
        .chain(tiles.last().zip(tiles.first()));
    for (tile1, tile2) in iter {
        if tile1.0 == tile2.0 {
            // Horizontal line
            for y in tile1.1.min(tile2.1)..=tile1.1.max(tile2.1) {
                grid[tile1.0][y] = GridTile::Edge;
            }
        } else {
            for x in tile1.0.min(tile2.0)..=tile1.0.max(tile2.0) {
                grid[x][tile1.1] = GridTile::Edge
            }
        }
    }
    grid
}

/// Raycast to find a coordinate inside the polygon. We find when we
/// are inside by counting the number of edges encountered so far
/// (odd = inside), but because we might still be on an outer edge,
/// we check both horizontal and vertical rays to see if we are inside.
/// Assumes we have not yet computed the inside.
fn raycast_inside_polygon(grid: &Grid) -> Option<(usize, usize)> {
    for x in 0usize..grid.len() {
        let mut horizontal_num_edges = 0u32;
        for y in 0usize..grid[x].len() {
            if grid[x][y] != GridTile::Outside {
                horizontal_num_edges += 1;
            } else if !horizontal_num_edges.is_multiple_of(2) {
                let vertical_num_edges = (0..x)
                    .map(|rx| {
                        if grid[rx][y] != GridTile::Outside {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>();
                if !vertical_num_edges.is_multiple_of(2) {
                    return Some((x, y));
                }
            }
        }
    }
    None
}

/// When we know where somewhere "inside" the polygon is, we can then
/// just iteratively flood-fill it to find all (compressed) coordinates
/// that are located within the polygon.
fn flood_fill(pos: (usize, usize), grid: &mut Grid) {
    let mut queue = BTreeSet::from([pos]);
    while let Some((x, y)) = queue.pop_first() {
        grid[x][y] = GridTile::Inside;
        queue.extend(neighbours((x, y)).filter(|&(nx, ny)| grid[nx][ny] == GridTile::Outside));
    }
}

/// We can quickly determine if a rectangle is inside the polygon by
/// checking that all coordinates on its edges are within the polygon.
fn rect_in_polygon(p1: (usize, usize), p2: (usize, usize), grid: &Grid) -> bool {
    for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
        if grid[x][p1.1] == GridTile::Outside || grid[x][p2.1] == GridTile::Outside {
            return false;
        }
    }
    for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
        if grid[p1.0][y] == GridTile::Outside || grid[p2.0][y] == GridTile::Outside {
            return false;
        }
    }
    true
}

// Because we work in compressed coordinates, we must uncompress them first
// before we calculate the area of each rectangle
fn uncompressed_rect_area(
    p1: (usize, usize),
    p2: (usize, usize),
    maps: (&CoordRMap, &CoordRMap),
) -> u64 {
    let p1 = (*maps.0.get(&p1.0).unwrap(), *maps.1.get(&p1.1).unwrap());
    let p2 = (*maps.0.get(&p2.0).unwrap(), *maps.1.get(&p2.1).unwrap());
    (p1.0.abs_diff(p2.0) as u64 + 1) * (p1.1.abs_diff(p2.1) as u64 + 1)
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 09/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<(usize, usize)> = contents
        .split("\n")
        .map(|s| {
            s.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let x_coords: Vec<usize> = data.iter().map(|t| t.0).collect();
    let y_coords: Vec<usize> = data.iter().map(|t| t.1).collect();
    let (mut x_map, mut x_rmap) = compress_coords(&x_coords, 0);
    let (mut y_map, mut y_rmap) = compress_coords(&y_coords, 0);
    let mut tiles: Vec<(usize, usize)> = data
        .iter()
        .map(|(x, y)| (*x_map.get(x).unwrap(), *y_map.get(y).unwrap()))
        .collect();

    let dims = (
        x_map.values().max().unwrap() + 1,
        y_map.values().max().unwrap() + 1,
    );
    let mut grid = rasterize_edges(dims, &tiles);
    let mut inside = raycast_inside_polygon(&grid);

    // For small edge-case inputs (e.g. the test input), the maximally-compressed
    // coordinate space (without any gaps) may leave us unable to find an "inside"
    // coordinate that originally existed, due to all the edges being compressed
    // together. We can solve this by adding a gap of 1 between each compressed
    // coordinate. But this 4x multiples the polygon search space, so instead we
    // always try to use maximal compression first (which should work for most
    // real inputs), and only use the less-optimal general case as a fallback.
    if inside.is_none() {
        (x_map, x_rmap) = compress_coords(&x_coords, 1);
        (y_map, y_rmap) = compress_coords(&y_coords, 1);
        tiles = data
            .into_iter()
            .map(|(x, y)| (*x_map.get(&x).unwrap(), *y_map.get(&y).unwrap()))
            .collect();
        let dims = (
            x_map.values().max().unwrap() + 1,
            y_map.values().max().unwrap() + 1,
        );
        grid = rasterize_edges(dims, &tiles);
        inside = raycast_inside_polygon(&grid);
    }

    let inside = inside.expect("Could not find a coord inside the polygon");
    flood_fill(inside, &mut grid);

    // Part 1 - compute the max rectangle area across all tile pairs
    type Pair = ((usize, usize), (usize, usize));
    let pairs: Vec<Pair> = tiles.into_iter().tuple_combinations().collect();
    let areas: Vec<u64> = pairs
        .iter()
        .map(|&(p1, p2)| uncompressed_rect_area(p1, p2, (&x_rmap, &y_rmap)))
        .collect();
    println!("Problem 17: {}", areas.iter().max().unwrap());

    // Part 2 - compute the max rectangle area only for tile pairs
    // that form polygons that lay entirely within (including borders)
    // the polygon.
    let areas: Vec<u64> = areas
        .into_iter()
        .zip(pairs.iter())
        .filter(|(_, pair)| rect_in_polygon(pair.0, pair.1, &grid))
        .map(|(area, _)| area)
        .collect();
    println!("Problem 18: {}", areas.into_iter().max().unwrap());
}
