use std::fs::read_to_string;

fn main() {
    let mut i1: i128 = 0;
    let mut j1: i128 = 0;
    let mut contour1: i128 = 2;
    let mut coordinates1: Vec<(i128, i128)> = Vec::new();
    coordinates1.push((0, 0));

    let mut i2: i128 = 0;
    let mut j2: i128 = 0;
    let mut contour2: i128 = 2;
    let mut coordinates2: Vec<(i128, i128)> = Vec::new();
    coordinates2.push((0, 0));

    for line in read_to_string("input/18").unwrap().lines() {
        let c: Vec<&str> = line.split(" ").collect();
        let n1 = c[1].parse::<i128>().unwrap();
        contour1 += n1;
        match c[0] {
            "U" => {
                coordinates1.push((i1 - n1, j1));
                i1 -= n1;
            }
            "D" => {
                coordinates1.push((i1 + n1, j1));
                i1 += n1;
            }
            "L" => {
                coordinates1.push((i1, j1 - n1));
                j1 -= n1;
            }
            "R" => {
                coordinates1.push((i1, j1 + n1));
                j1 += n1;
            }
            _ => unreachable!(),
        }
        let n2 = i128::from_str_radix(&c[2][2..c[2].len() - 2], 16).unwrap();
        contour2 += n2;
        match &c[2][c[2].len() - 2..] {
            "3)" => {
                coordinates2.push((i2 - n2, j2));
                i2 -= n2;
            }
            "1)" => {
                coordinates2.push((i2 + n2, j2));
                i2 += n2;
            }
            "2)" => {
                coordinates2.push((i2, j2 - n2));
                j2 -= n2;
            }
            "0)" => {
                coordinates2.push((i2, j2 + n2));
                j2 += n2;
            }
            _ => unreachable!(),
        }
    }

    println!(
        "Part 1: {}",
        (coordinates1
            .windows(2)
            .fold(0, |acc, w| acc + (w[0].0 + w[1].0) * (w[0].1 - w[1].1))
            + contour1)
            / 2
    );

    println!(
        "Part 2: {}",
        (coordinates2
            .windows(2)
            .fold(0, |acc, w| acc + (w[0].0 + w[1].0) * (w[0].1 - w[1].1))
            + contour2)
            / 2
    );
}
