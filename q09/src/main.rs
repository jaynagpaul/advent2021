use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> u32 {
    let mut heatmap = vec![];
    for line in input.lines() {
        let mut v = vec![];
        for c in line.chars() {
            v.push(c.to_digit(10).unwrap());
        }

        heatmap.push(v);
    }
    let mut tot = 0;

    for i in 0..heatmap.len() {
        for j in 0..heatmap[i].len() {
            if is_low_point(i, j, &heatmap) {
                tot += heatmap[i][j] + 1;
            }
        }
    }

    tot
}

fn part2(input: &str) -> i32 {
    let mut heatmap = vec![];
    for line in input.lines() {
        let mut v = vec![];
        for c in line.chars() {
            v.push(c.to_digit(10).unwrap());
        }

        heatmap.push(v);
    }

    let mut tot = 0;

    let mut low_points = vec![];
    for i in 0..heatmap.len() {
        for j in 0..heatmap[i].len() {
            if is_low_point(i, j, &heatmap) {
                low_points.push((i, j))
            }
        }
    }

    let mut basin_sizes = vec![];
    for (i, j) in low_points {
        let size = walk_basin(i, j, &heatmap);
        basin_sizes.push(size);
    }

    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn walk_basin(i: usize, j: usize, heatmap: &[Vec<u32>]) -> i32 {
    let mut visited = HashSet::new();
    let mut to_visit = vec![(i, j)];
    let mut size = 0;

    while let Some(pt) = to_visit.pop() {
        size += 1;
        let parent = heatmap[pt.0][pt.1];
        for adj_pt in adj_points(pt.0, pt.1, &heatmap) {
            if !visited.contains(&adj_pt)
                && heatmap[adj_pt.0][adj_pt.1] > parent
                && heatmap[adj_pt.0][adj_pt.1] != 9
            {
                to_visit.push(adj_pt);
                visited.insert(adj_pt);
            }
        }
    }

    size
}

fn adj_points(i: usize, j: usize, heatmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut adj = vec![];

    if i > 0 {
        adj.push((i - 1, j));
    }
    if i < heatmap.len() - 1 {
        adj.push((i + 1, j));
    }
    if j > 0 {
        adj.push((i, j - 1));
    }
    if j < heatmap[i].len() - 1 {
        adj.push((i, j + 1));
    }

    adj
}

fn is_low_point(i: usize, j: usize, heatmap: &[Vec<u32>]) -> bool {
    // Check the value of the current cell
    let current = heatmap[i][j];

    // Check the value of the adjacent cells
    if i > 0 && heatmap[i - 1][j] <= current {
        return false;
    }

    if i < heatmap.len() - 1 && heatmap[i + 1][j] <= current {
        return false;
    }

    if j > 0 && heatmap[i][j - 1] <= current {
        return false;
    }

    if j < heatmap[i].len() - 1 && heatmap[i][j + 1] <= current {
        return false;
    }

    true
}
