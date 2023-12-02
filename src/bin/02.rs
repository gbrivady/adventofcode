use sscanf::sscanf;
use std::fs::read_to_string;

const NB_RED: u32 = 12;
const NB_GREEN: u32 = 13;
const NB_BLUE: u32 = 14;

fn main() {
    let mut sum_p1: u32 = 0;
    let mut sum_p2: u32 = 0;

    for line in read_to_string("input/02").unwrap().lines() {
        let game: Vec<&str> = line.split(":").collect();
        let game_id: u32 = game[0][5..].parse::<u32>().unwrap();

        let mut nb_r_set: Vec<u32> = Vec::new();
        let mut nb_g_set: Vec<u32> = Vec::new();
        let mut nb_b_set: Vec<u32> = Vec::new();

        for set in game[1].split(";") {
            for cubes in set.split(",") {
                let (nb, color) = sscanf!(cubes, " {} {}", u32, str).unwrap();
                match color {
                    "red" => nb_r_set.push(nb),
                    "green" => nb_g_set.push(nb),
                    "blue" => nb_b_set.push(nb),
                    _ => panic!("UNEXPECTED COLOR"),
                }
            }
        }
        let (max_r, max_g, max_b) = (
            *nb_r_set.iter().max().unwrap_or(&0),
            *nb_g_set.iter().max().unwrap_or(&0),
            *nb_b_set.iter().max().unwrap_or(&0),
        );

        if max_r <= NB_RED && max_g <= NB_GREEN && max_b <= NB_BLUE {
            sum_p1 += game_id;
        }
        sum_p2 += max_r * max_g * max_b;
    }
    println!("Part 1: {sum_p1}\nPart 2: {sum_p2}");
}
