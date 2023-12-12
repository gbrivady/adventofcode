use std::cmp::{max, min};
use std::fs::read_to_string;

fn get_sum_shortest_path(
    galaxies: &Vec<(usize, usize)>,
    expansion_rate: u128,
    expansion_coords: (&Vec<usize>, &Vec<usize>),
) -> u128 {
    galaxies.iter().enumerate().fold(0, |acc, (i, (x1, y1))| {
        acc + galaxies.iter().skip(i).fold(0, |acc, (x2, y2)| {
            acc + {
                (min(*x1, *x2)..max(*x1, *x2)).fold(0, |acc, x| {
                    acc + {
                        if expansion_coords.0.contains(&x) {
                            expansion_rate
                        } else {
                            1
                        }
                    }
                }) + (min(*y1, *y2)..max(*y1, *y2)).fold(0, |acc, y| {
                    acc + {
                        if expansion_coords.1.contains(&y) {
                            expansion_rate
                        } else {
                            1
                        }
                    }
                })
            }
        })
    })
}

fn main() {
    let mut space: Vec<Vec<char>> = Vec::new();
    let mut galaxies_pos: Vec<(usize, usize)> = Vec::new();

    for (i, line) in read_to_string("input/11").unwrap().lines().enumerate() {
        space.push(
            line.chars()
                .enumerate()
                .map(|(j, cell)| {
                    if cell == '#' {
                        galaxies_pos.push((i, j));
                    }
                    cell
                })
                .collect(),
        );
    }

    let n = space.len();
    let m = space[0].len();

    let mut space_expansion_v: Vec<usize> = Vec::new();
    let mut space_expansion_h: Vec<usize> = Vec::new();

    for i in 0..n {
        if (0..m).fold(true, |acc, j| acc && (space[i][j] == '.')) {
            space_expansion_h.push(i);
        }
    }
    for j in 0..m {
        if (0..n).fold(true, |acc, i| acc && (space[i][j] == '.')) {
            space_expansion_v.push(j);
        }
    }

    println!(
        "Part 1: {}",
        get_sum_shortest_path(&galaxies_pos, 2, (&space_expansion_h, &space_expansion_v))
    );
    println!(
        "Part 2: {}",
        get_sum_shortest_path(
            &galaxies_pos,
            1000000,
            (&space_expansion_h, &space_expansion_v)
        )
    );
}
