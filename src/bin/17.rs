use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node<const T: usize> {
    i: usize,
    j: usize,
    p_dirs: [Direction; T],
}

fn get_neighbours<const T: usize>(
    node: Node<T>,
    min_move: usize,
    input: &Vec<Vec<u32>>,
) -> Vec<Node<T>> {
    let mut neighbours: Vec<Node<T>> = Vec::new();

    let mut potential_dirs = [true, true, true, true];

    if node.p_dirs.iter().all_equal() {
        match node.p_dirs[0] {
            Direction::None => (),
            Direction::Up => {
                potential_dirs[0] = false;
            }
            Direction::Down => {
                potential_dirs[1] = false;
            }
            Direction::Left => {
                potential_dirs[2] = false;
            }
            Direction::Right => {
                potential_dirs[3] = false;
            }
        }
    }
    if !node.p_dirs.iter().rev().take(min_move).all_equal() {
        match node.p_dirs[T - 1] {
            Direction::None => (),
            Direction::Up => potential_dirs = [true, false, false, false],
            Direction::Down => potential_dirs = [false, true, false, false],
            Direction::Left => potential_dirs = [false, false, true, false],
            Direction::Right => potential_dirs = [false, false, false, true],
        }
    }
    if node.i == 0 {
        potential_dirs[0] = false;
    }
    if node.i == input.len() - 1 {
        potential_dirs[1] = false;
    }
    if node.j == 0 {
        potential_dirs[2] = false;
    }
    if node.j == input[0].len() - 1 {
        potential_dirs[3] = false;
    }
    match node.p_dirs[T - 1] {
        Direction::None => (),
        Direction::Up => potential_dirs[1] = false,
        Direction::Down => potential_dirs[0] = false,
        Direction::Left => potential_dirs[3] = false,
        Direction::Right => potential_dirs[2] = false,
    }

    let mut next_dirs = [Direction::None; T];
    for i in 0..(T - 1) {
        next_dirs[i] = node.p_dirs[i + 1];
    }

    if potential_dirs[0] {
        neighbours.push(Node::<T> {
            i: node.i - 1,
            j: node.j,
            p_dirs: {
                let mut d = next_dirs.clone();
                d[T - 1] = Direction::Up;
                d
            },
        });
    }
    if potential_dirs[1] {
        neighbours.push(Node::<T> {
            i: node.i + 1,
            j: node.j,
            p_dirs: {
                let mut d = next_dirs.clone();
                d[T - 1] = Direction::Down;
                d
            },
        });
    }
    if potential_dirs[2] {
        neighbours.push(Node::<T> {
            i: node.i,
            j: node.j - 1,
            p_dirs: {
                let mut d = next_dirs.clone();
                d[T - 1] = Direction::Left;
                d
            },
        });
    }
    if potential_dirs[3] {
        neighbours.push(Node::<T> {
            i: node.i,
            j: node.j + 1,
            p_dirs: {
                let mut d = next_dirs.clone();
                d[T - 1] = Direction::Right;
                d
            },
        });
    }
    return neighbours;
}

fn get_dist<const T: usize>(
    start: (usize, usize),
    end: (usize, usize),
    min_move: usize,
    input: &Vec<Vec<u32>>,
) -> u32 {
    let mut distances: HashMap<Node<T>, u32> = HashMap::new();
    let mut vertex_priority: BinaryHeap<Reverse<(u32, Node<T>)>> = BinaryHeap::new();

    distances.insert(
        Node::<T> {
            i: start.0,
            j: start.1,
            p_dirs: [Direction::None; T],
        },
        input[start.0][start.1],
    );
    vertex_priority.push(Reverse((
        input[start.0][start.1],
        Node::<T> {
            i: start.0,
            j: start.1,
            p_dirs: [Direction::None; T],
        },
    )));

    while let Some(Reverse((distance, node))) = vertex_priority.pop() {
        if node.i == end.0 && node.j == end.0 && node.p_dirs.iter().rev().take(min_move).all_equal()
        {
            return distance;
        }
        if distance > *distances.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        for n in get_neighbours::<T>(node, min_move, &input) {
            let d = distance + input[node.i][node.j];
            if d < *distances.get(&n).unwrap_or(&u32::MAX) {
                vertex_priority.push(Reverse((d, n)));
                distances.insert(n, d);
            }
        }
    }

    return u32::MAX;
}

fn main() {
    let input: Vec<Vec<u32>> = read_to_string("input/17")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!(
        "Part 1: {}",
        get_dist::<3>((0, 0), (input.len() - 1, input[0].len() - 1), 1, &input)
    );
    println!(
        "Part 2: {}",
        get_dist::<10>((0, 0), (input.len() - 1, input[0].len() - 1), 4, &input)
    );
}
