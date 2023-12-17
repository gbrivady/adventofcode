use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rotate_ray(direction: Direction, mirror_lr: bool) -> Direction {
    if mirror_lr {
        // Case /
        match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    } else {
        // Case \
        match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn simulate_beam(
    i: usize,
    j: usize,
    direction: Direction,
    cache: &mut HashSet<(usize, usize, Direction)>,
    input: &Vec<Vec<char>>,
    grid: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize, Direction)> {
    grid[i][j] = true;

    let new_directions = match input[i][j] {
        '.' => vec![direction],
        '/' => vec![rotate_ray(direction, true)],
        '\\' => vec![rotate_ray(direction, false)],
        '-' => {
            if direction == Direction::Left || direction == Direction::Right {
                vec![direction]
            } else {
                vec![Direction::Left, Direction::Right]
            }
        }
        '|' => {
            if direction == Direction::Up || direction == Direction::Down {
                vec![direction]
            } else {
                vec![Direction::Up, Direction::Down]
            }
        }
        _ => unreachable!(),
    };
    let mut to_call: Vec<(usize, usize, Direction)> = Vec::new();
    for d in new_directions {
        match d {
            Direction::Up => {
                if i > 0 && !cache.contains(&(i - 1, j, d)) {
                    cache.insert((i - 1, j, d));
                    to_call.push((i - 1, j, d));
                }
            }
            Direction::Down => {
                if i < grid.len() - 1 && !cache.contains(&(i + 1, j, d)) {
                    cache.insert((i + 1, j, d));
                    to_call.push((i + 1, j, d));
                }
            }
            Direction::Left => {
                if j > 0 && !cache.contains(&(i, j - 1, d)) {
                    cache.insert((i, j - 1, d));
                    to_call.push((i, j - 1, d));
                }
            }
            Direction::Right => {
                if j < grid[0].len() - 1 && !cache.contains(&(i, j + 1, d)) {
                    cache.insert((i, j + 1, d));
                    to_call.push((i, j + 1, d));
                }
            }
        }
    }
    return to_call;
}

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input/16")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n = input.len();
    let m = input[0].len();
    let mut results: Vec<u32> = Vec::new();
    for i in 0..n {
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let mut energized: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
            let mut cache: HashSet<(usize, usize, Direction)> = HashSet::new();
            cache.insert((i, 0, d));
            let mut to_call = vec![(i, 0, d)];
            while !to_call.is_empty() {
                let mut new_calls: Vec<(usize, usize, Direction)> = Vec::new();
                for (i, j, direction) in to_call.iter() {
                    new_calls.append(&mut simulate_beam(
                        *i,
                        *j,
                        *direction,
                        &mut cache,
                        &input,
                        &mut energized,
                    ));
                }
                to_call = new_calls;
            }
            results.push(energized.iter().fold(0, |acc, line| {
                acc + line.iter().fold(0, |acc, cell| {
                    acc + {
                        if *cell {
                            1
                        } else {
                            0
                        }
                    }
                })
            }));
        }
    }
    for i in 0..n {
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let mut energized: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
            let mut cache: HashSet<(usize, usize, Direction)> = HashSet::new();
            cache.insert((i, m - 1, d));
            let mut to_call = vec![(i, m - 1, d)];
            while !to_call.is_empty() {
                let mut new_calls: Vec<(usize, usize, Direction)> = Vec::new();
                for (i, j, direction) in to_call.iter() {
                    new_calls.append(&mut simulate_beam(
                        *i,
                        *j,
                        *direction,
                        &mut cache,
                        &input,
                        &mut energized,
                    ));
                }
                to_call = new_calls;
            }
            results.push(energized.iter().fold(0, |acc, line| {
                acc + line.iter().fold(0, |acc, cell| {
                    acc + {
                        if *cell {
                            1
                        } else {
                            0
                        }
                    }
                })
            }));
        }
    }
    for j in 0..m {
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let mut energized: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
            let mut cache: HashSet<(usize, usize, Direction)> = HashSet::new();
            cache.insert((0, j, d));
            let mut to_call = vec![(0, j, d)];
            while !to_call.is_empty() {
                let mut new_calls: Vec<(usize, usize, Direction)> = Vec::new();
                for (i, j, direction) in to_call.iter() {
                    new_calls.append(&mut simulate_beam(
                        *i,
                        *j,
                        *direction,
                        &mut cache,
                        &input,
                        &mut energized,
                    ));
                }
                to_call = new_calls;
            }
            results.push(energized.iter().fold(0, |acc, line| {
                acc + line.iter().fold(0, |acc, cell| {
                    acc + {
                        if *cell {
                            1
                        } else {
                            0
                        }
                    }
                })
            }));
        }
    }
    for j in 0..m {
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let mut energized: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
            let mut cache: HashSet<(usize, usize, Direction)> = HashSet::new();
            cache.insert((n - 1, j, d));
            let mut to_call = vec![(n - 1, j, d)];
            while !to_call.is_empty() {
                let mut new_calls: Vec<(usize, usize, Direction)> = Vec::new();
                for (i, j, direction) in to_call.iter() {
                    new_calls.append(&mut simulate_beam(
                        *i,
                        *j,
                        *direction,
                        &mut cache,
                        &input,
                        &mut energized,
                    ));
                }
                to_call = new_calls;
            }
            results.push(energized.iter().fold(0, |acc, line| {
                acc + line.iter().fold(0, |acc, cell| {
                    acc + {
                        if *cell {
                            1
                        } else {
                            0
                        }
                    }
                })
            }));
        }
    }

    println!("Part 1: {}", results[2]);
    println!("Part 2: {}", results.iter().max().unwrap());
}
