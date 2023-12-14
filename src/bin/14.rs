use std::fs::read_to_string;

fn run_cycle(grid: &mut Vec<Vec<char>>) {
    //North
    for i in 1..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while (k > 0) && (grid[k - 1][j] == '.') {
                k -= 1;
            }
            grid[i][j] = '.';
            grid[k][j] = 'O';
        }
    }

    // West
    for j in 1..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = j;
            while (k > 0) && (grid[i][k - 1] == '.') {
                k -= 1;
            }
            grid[i][j] = '.';
            grid[i][k] = 'O';
        }
    }

    // South
    for i in (0..(grid.len() - 1)).rev() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while (k < grid.len() - 1) && (grid[k + 1][j] == '.') {
                k += 1;
            }
            grid[i][j] = '.';
            grid[k][j] = 'O';
        }
    }

    // East
    for j in (0..(grid[0].len() - 1)).rev() {
        for i in 0..grid.len() {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = j;
            while (k < grid[0].len() - 1) && (grid[i][k + 1] == '.') {
                k += 1;
            }
            grid[i][j] = '.';
            grid[i][k] = 'O';
        }
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = read_to_string("input/14")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut grid_p1: Vec<Vec<char>> = grid.iter().map(|line| line.to_vec()).collect();
    for i in 1..grid_p1.len() {
        for j in 0..grid_p1[0].len() {
            if grid_p1[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while (k > 0) && (grid_p1[k - 1][j] == '.') {
                k -= 1;
            }
            grid_p1[i][j] = '.';
            grid_p1[k][j] = 'O';
        }
    }
    println!(
        "Part 1: {}",
        grid_p1.iter().enumerate().fold(0, |acc, (i, line)| acc
            + line.iter().fold(0, |acc, c| {
                if *c == 'O' {
                    acc + (grid_p1.len() - i)
                } else {
                    acc
                }
            }))
    );

    let mut loads: Vec<usize> = Vec::new();
    loads.push(grid.iter().enumerate().fold(0, |acc, (i, line)| {
        acc + line.iter().fold(0, |acc, c| {
            if *c == 'O' {
                acc + (grid.len() - i)
            } else {
                acc
            }
        })
    }));

    'outer: loop {
        run_cycle(&mut grid);
        let l = grid.iter().enumerate().fold(0, |acc, (i, line)| {
            acc + line.iter().fold(0, |acc, c| {
                if *c == 'O' {
                    acc + (grid.len() - i)
                } else {
                    acc
                }
            })
        });
        loads.push(l);
        for t in 10..(loads.len() / 2) {
            if (0..t).all(|i| loads[loads.len() - 1 - i] == loads[loads.len() - t - 1 - i]) {
                println!("Part 2: Found period of length {t}");
                println!(
                    "\tResult: {}",
                    loads[((1000000000 - (loads.len() as u32 - 2 * t as u32)) % t as u32) as usize
                        + (loads.len() - t)]
                );
                break 'outer;
            }
        }
    }
}
