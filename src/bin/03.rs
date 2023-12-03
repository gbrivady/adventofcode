use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn check_neighbours(
    i: usize,
    j: usize,
    l: usize,
    engine: &Vec<Vec<char>>,
    n: usize,
    m: usize,
) -> (bool, Vec<(usize, usize)>) {
    let mut is_part_number = false;
    let mut gear_symbol_pos: Vec<(usize, usize)> = Vec::new();

    let x_range = {
        if j == 0 {
            0
        } else {
            j - 1
        }
    }..={
        if j + l < m {
            j + l
        } else {
            j + l - 1
        }
    };
    let mut y_values: Vec<usize> = Vec::new();
    if i >= 1 {
        y_values.push(i - 1);
    }
    if i + 1 < n {
        y_values.push(i + 1);
    }

    for (x, y) in x_range.cartesian_product(y_values) {
        is_part_number |= engine[y][x] != '.' && !engine[y][x].is_numeric();
        if engine[y][x] == '*' {
            gear_symbol_pos.push((y, x));
        }
    }
    if j >= 1 {
        is_part_number |= engine[i][j - 1] != '.' && !engine[i][j - 1].is_numeric();
        if engine[i][j - 1] == '*' {
            gear_symbol_pos.push((i, j - 1));
        }
    }
    if j + l < m {
        is_part_number |= engine[i][j + l] != '.' && !engine[i][j + l].is_numeric();
        if engine[i][j + l] == '*' {
            gear_symbol_pos.push((i, j + l));
        }
    }
    return (is_part_number, gear_symbol_pos);
}

fn main() {
    let mut sum_p1 = 0;

    let mut engine: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("input/03").unwrap().lines() {
        engine.push(line.chars().collect());
    }

    let n = engine.len();
    let m = engine[0].len();

    let mut potential_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..n {
        let mut j = 0;
        while j < m {
            match engine[i][j].to_digit(10) {
                Some(d) => {
                    // Begin to read a number
                    let mut l: usize = 1;
                    let mut part_num = d;
                    while j + l < m {
                        match engine[i][j + l].to_digit(10) {
                            Some(d) => {
                                part_num *= 10;
                                part_num += d;
                                l += 1;
                            }
                            None => break, // End of number
                        }
                    }
                    let r = check_neighbours(i, j, l, &engine, n, m);
                    if r.0 {
                        sum_p1 += part_num;
                    }
                    for (x, y) in r.1 {
                        match potential_gears.get_mut(&(x, y)) {
                            Some(v) => v.push(part_num),
                            None => {
                                potential_gears.insert((x, y), vec![part_num]);
                            }
                        }
                    }
                    j += l;
                }
                None => j += 1,
            }
        }
    }
    let mut sum_p2 = 0;
    for (_, parts_num) in potential_gears {
        if parts_num.len() == 2 {
            sum_p2 += parts_num[0] * parts_num[1];
        }
    }

    println!("Part 1: {sum_p1}\nPart 2: {sum_p2}");
}
