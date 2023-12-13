use std::fs::read_to_string;

fn check_horizontal_reflection(pattern: &Vec<Vec<char>>, nb_smudges: u32) -> usize {
    for i in 1..pattern.len() {
        if (1..=i)
            .take_while(|k| i + *k <= pattern.len())
            .fold(0, |acc, k| {
                acc + pattern[i - k]
                    .iter()
                    .zip(pattern[i + k - 1].iter())
                    .fold(0, |acc, (c1, c2)| if *c1 != *c2 { acc + 1 } else { acc })
            })
            == nb_smudges
        {
            return i;
        }
    }
    return usize::MAX;
}

fn check_vertical_reflection(pattern: &Vec<Vec<char>>, nb_smudges: u32) -> usize {
    for j in 1..pattern[0].len() {
        if (1..=j)
            .take_while(|k| j + *k <= pattern[0].len())
            .fold(0, |acc, k| {
                acc + (0..pattern.len()).fold(0, |acc, i| {
                    if pattern[i][j - k] != pattern[i][j + k - 1] {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
            == nb_smudges
        {
            return j;
        }
    }
    return usize::MAX;
}

fn main() {
    let mut all_patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut cur_pattern: Vec<Vec<char>> = Vec::new();

    for line in read_to_string("input/13").unwrap().lines() {
        let c: Vec<char> = line.chars().collect();
        match c.is_empty() {
            true => {
                all_patterns.push(cur_pattern);
                cur_pattern = Vec::new();
            }
            _ => cur_pattern.push(c),
        }
    }
    all_patterns.push(cur_pattern);
    println!(
        "Part 1: {}",
        all_patterns.iter().fold(0, |acc, p| {
            let h = check_horizontal_reflection(p, 0);
            let v = check_vertical_reflection(p, 0);
            if h != usize::MAX {
                acc + h * 100
            } else if v != usize::MAX {
                acc + v
            } else {
                unreachable!()
            }
        })
    );
    println!(
        "Part 2: {}",
        all_patterns.iter().fold(0, |acc, p| {
            let h = check_horizontal_reflection(p, 1);
            let v = check_vertical_reflection(p, 1);
            if h != usize::MAX {
                acc + h * 100
            } else if v != usize::MAX {
                acc + v
            } else {
                unreachable!()
            }
        })
    );
}
