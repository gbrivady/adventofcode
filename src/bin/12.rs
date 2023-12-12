use std::collections::HashMap;
use std::fs::read_to_string;

fn read_conditions(
    records: &Vec<usize>,
    conditions: &Vec<char>,
    cache: &mut HashMap<(Vec<usize>, Vec<char>), u128>,
) -> u128 {
    if conditions.is_empty() {
        if records.is_empty() {
            return 1;
        }
        return 0;
    }

    match conditions[0] {
        '.' => read_conditions(records, &conditions[1..].to_vec(), cache),
        '#' => try_fit_record(records, conditions, cache),
        '?' => {
            read_conditions(records, &conditions[1..].to_vec(), cache)
                + try_fit_record(records, conditions, cache)
        }
        _ => unreachable!(),
    }
}

fn try_fit_record(
    records: &Vec<usize>,
    conditions: &Vec<char>,
    cache: &mut HashMap<(Vec<usize>, Vec<char>), u128>,
) -> u128 {
    if records.is_empty() {
        return 0;
    }

    let spring = records[0];
    if conditions.len() < spring {
        return 0;
    }

    if (0..spring).any(|i| conditions[i] == '.') {
        return 0;
    }
    if conditions.len() == spring {
        if records.len() == 1 {
            return 1;
        }
        return 0;
    }
    if conditions[spring] == '#' {
        return 0;
    }
    match cache.get(&(records.clone(), conditions.clone())) {
        Some(n) => return *n,
        None => (),
    };
    let res = read_conditions(
        &records[1..].to_vec(),
        &conditions[(spring + 1)..].to_vec(),
        cache,
    );
    cache.insert((records.clone(), conditions.clone()), res);
    return res;
}

fn main() {
    let mut cache: HashMap<(Vec<usize>, Vec<char>), u128> = HashMap::new();

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    for line in read_to_string("input/12").unwrap().lines() {
        //parse line to spring
        let mut line_it = line.split(" ");

        // parse record
        let conditions: Vec<char> = line_it.next().unwrap().chars().collect();
        //parse springs groups
        let spring_groups: Vec<usize> = line_it
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        sum_p1 += read_conditions(&spring_groups, &conditions, &mut cache);

        let mut conditions_p2 = conditions.to_vec();
        let mut spring_groups_p2 = spring_groups.to_vec();

        for _ in 0..4 {
            conditions_p2.push('?');
            conditions_p2.extend(&conditions);
            spring_groups_p2.extend(&spring_groups);
        }

        sum_p2 += read_conditions(&spring_groups_p2, &conditions_p2, &mut cache);
    }

    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
}
