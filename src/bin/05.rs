use sscanf::sscanf;
use std::cmp::min;
use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input/05")
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect();

    let seed_numbers: Vec<u64> = lines[0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

    for i in (0..seed_numbers.len()).step_by(2) {
        seed_ranges.push((seed_numbers[i], seed_numbers[i + 1]));
    }

    let mut i = 1;

    let mut mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    while i < lines.len() {
        if lines[i] != "" {
            // We have a mapping
            i += 1; // Mapping id (no care)
            let mut mapping: Vec<(u64, u64, u64)> = Vec::new();
            while i < lines.len() && lines[i] != "" {
                mapping.push(sscanf!(lines[i], "{} {} {}", u64, u64, u64).unwrap());
                i += 1;
            }
            mappings.push(mapping);
        } else {
            i += 1;
        }
    }

    let mut seed_positions: Vec<u64> = Vec::new();

    for s in seed_numbers {
        let mut m_of_s = s;
        for m in &mappings {
            for (a, b, r) in m {
                if m_of_s >= *b && m_of_s < *b + *r {
                    m_of_s -= *b;
                    m_of_s += *a;
                    break;
                }
            }
        }
        seed_positions.push(m_of_s);
    }
    println!("{}", seed_positions.iter().min().unwrap());

    let mut min_of_ranges: Vec<u64> = Vec::new();
    for r in seed_ranges {
        let mut old_ranges: Vec<(u64, u64)> = Vec::new();
        old_ranges.push(r);

        for m in mappings.iter() {
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            let mut i = 0;
            while i < old_ranges.len() {
                let (a, l) = old_ranges[i];
                if l == 0 {
                    i += 1;
                    continue;
                }
                let mut keep_range: bool = true;
                for (y, x, r) in m {
                    if a >= *x {
                        if a >= *x + *r {
                            continue;
                        } else {
                            if a + l <= *x + *r {
                                new_ranges.push(((a - *x) + *y, l));
                                keep_range = false;
                                break;
                            } else {
                                new_ranges.push(((a - *x) + *y, (*x + *r) - a));
                                old_ranges.push((*x + *r, (a + l) - (*x + *r)));
                                keep_range = false;
                                break;
                            }
                        }
                    }
                    if a + l <= *x {
                        continue;
                    }
                    if a + l < *x + *r {
                        old_ranges.push((a, *x - a));
                        new_ranges.push((*y, (a + l) - *x));
                        keep_range = false;
                        break;
                    } else {
                        new_ranges.push((*y, *r));
                        old_ranges.push((a, *x - a));
                        old_ranges.push((*x + *r, (a + l) - (*x + *r)));
                        keep_range = false;
                        break;
                    }
                }
                if keep_range {
                    new_ranges.push(old_ranges[i])
                }
                i += 1;
            }
            old_ranges = new_ranges;
        }
        let mut cur_min = u64::MAX;
        for (a, _) in old_ranges {
            cur_min = min(a, cur_min);
        }
        if cur_min == 0 {
            ();
        }
        min_of_ranges.push(cur_min);
    }
    println!("{}", min_of_ranges.iter().min().unwrap());
}
