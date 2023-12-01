use std::fs::read_to_string;

const DIGIT_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut result: (u32, u32) = (0, 0);

    for line in read_to_string("input/01").unwrap().lines() {
        let mut found_first: (bool, bool) = (false, false);
        let mut cur_digit: (u32, u32) = (0, 0);

        for (i, c) in line.char_indices() {
            match c.to_digit(10) {
                Some(digit) => {
                    cur_digit.0 = digit;
                    cur_digit.1 = digit;
                    if !found_first.0 {
                        found_first.0 = true;
                        result.0 += 10 * digit;
                    }
                    if !found_first.1 {
                        found_first.1 = true;
                        result.1 += 10 * digit;
                    }
                }

                None => {
                    for j in 0..9 {
                        if line[i..].starts_with(DIGIT_STR[j]) {
                            cur_digit.1 = (j + 1) as u32;
                            if !found_first.1 {
                                result.1 += cur_digit.1 * 10;
                                found_first.1 = true;
                            }
                            break;
                        }
                    }
                }
            }
        }
        result.0 += cur_digit.0;
        result.1 += cur_digit.1;
    }

    println!("Part 1: {}\nPart 2: {}", result.0, result.1);
}
