use std::collections::HashSet;
use std::fs::read_to_string;

fn walk_neighbours(i: usize, j: usize, grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

    let mut result: Vec<(usize, usize)> = Vec::new();

    if i > 0 && grid[i - 1][j] {
        result.push((i - 1, j));
    }
    if i < n - 1 && grid[i + 1][j] {
        result.push((i + 1, j));
    }
    if j > 0 && grid[i][j - 1] {
        result.push((i, j - 1));
    }
    if j < m - 1 && grid[i][j + 1] {
        result.push((i, j + 1));
    }
    return result;
}

fn main() {
    let mut start: (usize, usize) = (0, 0);
    let grid: Vec<Vec<bool>> = read_to_string("input/21")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        start = (i, j);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut cur_positions: HashSet<(usize, usize)> = HashSet::new();
    cur_positions.insert(start);

    for _ in 0..64 {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for p in cur_positions {
            for new_p in walk_neighbours(p.0, p.1, &grid) {
                new_positions.insert(new_p);
            }
        }
        cur_positions = new_positions;
    }
    println!("Part 1: {}", cur_positions.len());
}
