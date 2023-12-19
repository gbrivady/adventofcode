use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct InstructionMap {
    map_to: String,
    category: usize,
    less_than: bool,
    value: u32,
}

type Part = [u32; 4];

type Interval4D = [(u32, u32); 4];

const INTERVAL_NONE: Interval4D = [
    (u32::MAX, u32::MAX),
    (u32::MAX, u32::MAX),
    (u32::MAX, u32::MAX),
    (u32::MAX, u32::MAX),
];

fn split(interval: Interval4D, dim: usize, n: u32, lower: bool) -> (Interval4D, Interval4D) {
    let (i1, i2) = split_range(interval[dim].0, interval[dim].1, n, lower);
    if i1.0 == u32::MAX {
        return (INTERVAL_NONE, interval);
    } else if i2.0 == u32::MAX {
        return (interval, INTERVAL_NONE);
    } else {
        let (mut r1, mut r2) = (interval.clone(), interval.clone());
        r1[dim] = i1;
        r2[dim] = i2;
        return (r1, r2);
    }
}

fn split_range(a: u32, b: u32, n: u32, lower: bool) -> ((u32, u32), (u32, u32)) {
    if lower {
        if b < n {
            return ((a, b), (u32::MAX, u32::MAX));
        }
        if a < n {
            return ((a, n - 1), (n, b));
        }
        return ((u32::MAX, u32::MAX), (a, b));
    } else {
        if a > n {
            return ((a, b), (u32::MAX, u32::MAX));
        }
        if b > n {
            return ((n + 1, b), (a, n));
        }
        return ((u32::MAX, u32::MAX), (a, b));
    }
}

fn parse_instruction(raw: &str, instructions: &mut HashMap<String, Vec<InstructionMap>>) {
    let (id, mut raw_left) = sscanf!(raw, "{}{{{}}}", String, &str).unwrap();

    let mut mapping: Vec<InstructionMap> = Vec::new();
    loop {
        match sscanf!(raw_left, "{}{}{}:{},{}", char, char, u32, String, &str) {
            Err(_) => break,
            Ok((cat, ineq, value, id_map, raw)) => {
                raw_left = raw;
                let category = match cat {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                let less_than = match ineq {
                    '<' => true,
                    '>' => false,
                    _ => unreachable!(),
                };
                mapping.push(InstructionMap {
                    map_to: id_map,
                    category: category,
                    less_than: less_than,
                    value: value,
                });
            }
        }
    }

    mapping.push(InstructionMap {
        map_to: raw_left.to_string(),
        category: 0,
        less_than: true,
        value: u32::MAX,
    });
    instructions.insert(id, mapping);
}

fn do_workflow(p: Part, instructions: &HashMap<String, Vec<InstructionMap>>) -> u32 {
    let mut cur_id = "in".to_string();
    while cur_id != "R" && cur_id != "A" {
        for m in instructions.get(&cur_id).unwrap().iter() {
            if (m.less_than && p[m.category] < m.value) || (!m.less_than && p[m.category] > m.value)
            {
                cur_id = m.map_to.clone();
                break;
            }
        }
    }
    if cur_id == "A" {
        return p.iter().fold(0, |acc, c| acc + c);
    }
    return 0;
}

fn interval_through_instruction(
    init_id: String,
    init_interval: Interval4D,
    instructions: &HashMap<String, Vec<InstructionMap>>,
) -> u128 {
    let mut result: u128 = 0;
    let mut intervals_id_stack: Vec<(Interval4D, String)> = Vec::new();
    intervals_id_stack.push((init_interval, init_id.clone()));

    while !intervals_id_stack.is_empty() {
        let mut new_intervals_ids: Vec<(Interval4D, String)> = Vec::new();
        for (mut interval, id) in intervals_id_stack.iter() {
            if id == "R" {
                continue;
            }
            if id == "A" {
                result += interval
                    .iter()
                    .fold(1, |acc, i| acc * ((i.1 - i.0 + 1) as u128));
                continue;
            }
            for m in instructions.get(id).unwrap().iter() {
                let (i1, i2) = split(interval, m.category, m.value, m.less_than);
                if i1 != INTERVAL_NONE {
                    new_intervals_ids.push((i1, m.map_to.clone()));
                }
                if i2 != INTERVAL_NONE {
                    interval = i2;
                } else {
                    break;
                }
            }
        }
        intervals_id_stack = new_intervals_ids;
    }

    return result;
}

fn main() {
    let mut is_mappings = true;
    let mut instructions: HashMap<String, Vec<InstructionMap>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    for line in read_to_string("input/19").unwrap().lines() {
        if line == "" {
            is_mappings = false;
            continue;
        }
        if is_mappings {
            parse_instruction(line, &mut instructions);
        } else {
            parts.push({
                let (x, m, a, s) =
                    sscanf!(line, "{{x={},m={},a={},s={}}}", u32, u32, u32, u32).unwrap();
                [x, m, a, s]
            });
        }
    }

    println!(
        "Part 1: {}",
        parts
            .iter()
            .fold(0, |acc, p| acc + do_workflow(*p, &instructions))
    );

    println!(
        "Part 2: {}",
        interval_through_instruction(
            "in".to_string(),
            [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
            &instructions
        )
    );
}
