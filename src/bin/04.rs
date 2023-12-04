use std::fs::read_to_string;

const NB_CARD: usize = 201;

fn main() {
    let mut total_points = 0;
    let mut nb_copies: Vec<u32> = vec![0; NB_CARD];

    for (i, line) in read_to_string("input/04").unwrap().lines().enumerate() {
        let card_values: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect();

        nb_copies[i] += 1;

        let mut winning_values: Vec<u32> = Vec::new();
        for v in card_values[0].split(" ") {
            match v.parse::<u32>() {
                Ok(n) => winning_values.push(n),
                Err(_) => (),
            }
        }
        let mut my_values: Vec<u32> = Vec::new();
        for v in card_values[1].split(" ") {
            match v.parse::<u32>() {
                Ok(n) => my_values.push(n),
                Err(_) => (),
            }
        }

        let mut card_points: u32 = 0;
        let mut nb_match: usize = 0;

        for v in my_values {
            if winning_values.contains(&v) {
                nb_match += 1;
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        total_points += card_points;
        for j in {
            (i + 1)..={
                if i + nb_match > 200 {
                    200
                } else {
                    i + nb_match
                }
            }
        } {
            nb_copies[j] += nb_copies[i];
        }
    }

    println!("Part 1: {}", total_points);
    println!("Part 2: {}", nb_copies.iter().sum::<u32>());
}
