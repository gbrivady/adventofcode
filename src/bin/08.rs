use num::Integer;
use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::read_to_string;

fn hash_string(s: String) -> u32 {
    let mut h: u32 = 0;
    for c in s.chars() {
        h *= 100;
        h += (c.to_digit(36).unwrap() - 10) as u32;
    }
    h
}

fn main() {
    let mut instructions: Vec<i8> = Vec::new();
    let binding = read_to_string("input/08").unwrap();
    let mut line_it = binding.lines();
    for c in line_it.next().unwrap().chars() {
        match c {
            'L' => instructions.push(-1),
            'R' => instructions.push(1),
            _ => unreachable!(),
        }
    }
    line_it.next().unwrap();

    let mut map: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut starting_nodes: Vec<u32> = Vec::new();

    for line in line_it {
        let (a, b, c) = sscanf!(line, "{} = ({}, {})", String, String, String).unwrap();
        let h_a = hash_string(a);
        map.insert(h_a, (hash_string(b), hash_string(c)));
        if h_a % 100 == 0 {
            starting_nodes.push(h_a);
        }
    }

    let mut periods: Vec<u64> = Vec::new();

    for n in starting_nodes.iter() {
        let mut node = *n;
        let mut cur_instruction = 0;
        let mut nb_steps = 0;
        while node % 100 != 25 {
            if instructions[cur_instruction] == -1 {
                node = map.get(&node).unwrap().0;
            } else {
                node = map.get(&node).unwrap().1;
            }
            cur_instruction += 1;
            cur_instruction %= instructions.len();
            nb_steps += 1;
        }
        periods.push(nb_steps as u64);
    }

    println!(
        "Part 1: {}",
        periods[starting_nodes.iter().position(|&x| x == 0).unwrap()]
    );
    println!(
        "Part 2: {}",
        periods.iter().fold(1, |acc: u64, t| acc.lcm(&t))
    );
}
