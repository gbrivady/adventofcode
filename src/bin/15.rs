use std::fs::read_to_string;

fn hash(current_value: u32, c: char) -> u32 {
    ((current_value + (c as u32)) * 17) % 256
}

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input/15")
        .unwrap()
        .split_terminator(',')
        .map(|s| s.chars().collect())
        .collect();
    println!(
        "Part 1: {}",
        input
            .iter()
            .fold(0, |acc, s| acc + s.iter().fold(0, |acc, c| hash(acc, *c))),
    );

    //Part 2
    let mut boxes: Vec<Vec<(Vec<char>, u32)>> = vec![Vec::new(); 256];
    for s in input.iter() {
        if s[s.len() - 1] == '-' {
            let label: Vec<char> = s[..(s.len() - 1)].to_vec(); // label
            let box_nb = label.iter().fold(0, |acc, c| hash(acc, *c)); //box nb

            match boxes[box_nb as usize]
                .iter()
                .position(|(label_b, _)| label_b.iter().zip(label.iter()).all(|(a, b)| *a == *b))
            {
                Some(i) => {
                    boxes[box_nb as usize].remove(i);
                }
                None => (),
            }
        } else if s[s.len() - 1].is_numeric() {
            let power = s[s.len() - 1].to_digit(10).unwrap();
            let label: Vec<char> = s[..(s.len() - 2)].to_vec(); // label
            let box_nb = label.iter().fold(0, |acc, c| hash(acc, *c)); //box nb

            match boxes[box_nb as usize]
                .iter()
                .position(|(label_b, _)| label_b.iter().zip(label.iter()).all(|(a, b)| *a == *b))
            {
                Some(i) => {
                    boxes[box_nb as usize][i].1 = power;
                }
                None => boxes[box_nb as usize].push((label, power)),
            }
        }
    }

    println!(
        "Part 2: {}",
        boxes.iter().enumerate().fold(0, |acc, (i, b)| {
            acc + (i as u32 + 1)
                * b.iter()
                    .enumerate()
                    .fold(0, |acc, (i, (_, p))| acc + (i as u32 + 1) * *p)
        })
    );
}
