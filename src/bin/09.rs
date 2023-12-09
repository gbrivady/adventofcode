use std::fs::read_to_string;

fn main() {
    let sums = read_to_string("input/09")
        .unwrap()
        .lines()
        .fold((0, 0), |acc, line| {
            let sequence: Vec<i64> = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

            let mut sub_sequences: Vec<Vec<i64>> = Vec::new();
            sub_sequences.push(sequence);
            while !sub_sequences[sub_sequences.len() - 1]
                .iter()
                .fold(true, |acc, x| acc && *x == 0)
            {
                sub_sequences.push(
                    sub_sequences[sub_sequences.len() - 1]
                        .windows(2)
                        .map(|s| s[1] - s[0])
                        .collect(),
                );
            }
            //build the last value
            let (s1, s2) = sub_sequences.iter().rev().skip(1).fold((0, 0), |acc, seq| {
                (acc.0 + seq[seq.len() - 1], seq[0] - acc.1)
            });
            (acc.0 + s1, acc.1 + s2)
        });

    println!("Part 1: {}", sums.0);
    println!("Part 2: {}", sums.1);
}
