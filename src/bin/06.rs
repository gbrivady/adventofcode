use std::fs::read_to_string;

fn binary_search<F>(x0: u64, xmax: u64, y: u64, f: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut a = x0;
    let mut fa = f(a);
    let mut b = xmax;
    let mut fb = f(b);

    let mut nb_iter = 0;
    loop {
        if nb_iter > 10000 {
            break a;
        }
        if fa == y {
            break fa;
        }
        if fb == y {
            break fb;
        }
        let c = (a + b) / 2;
        let fc = f(c);
        nb_iter += 1;
        if fc >= y {
            b = c;
            fb = fc;
        } else {
            a = c;
            fa = fc;
        }
    }
}

fn main() {
    let lines: Vec<String> = read_to_string("input/06")
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect();

    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    for text in lines[0].split(" ") {
        match text.trim().parse::<u32>() {
            Ok(t) => times.push(t),
            Err(_) => (),
        }
    }
    for text in lines[1].split(" ") {
        match text.trim().parse::<u32>() {
            Ok(d) => distances.push(d),
            Err(_) => (),
        }
    }

    let mut nb_wins: Vec<u32> = Vec::new();
    // push button for k seconds : speed of k, travel distance of (t-k)k
    for i in 0..times.len() {
        let t = times[i];
        let record = distances[i];
        let mut counter = 0;
        for k in 0..t {
            if (t - k) * k > record {
                counter += 1;
            }
        }
        nb_wins.push(counter);
    }

    println!("{}", nb_wins.iter().fold(1, |acc, x| acc * x));
    // Part 2

    let mut time: u64 = 0;
    let mut record_distance: u64 = 0;

    for c in lines[0].chars() {
        match c.to_digit(10) {
            Some(d) => {
                time *= 10;
                time += d as u64;
            }
            None => (),
        }
    }
    for c in lines[1].chars() {
        match c.to_digit(10) {
            Some(d) => {
                record_distance *= 10;
                record_distance += d as u64;
            }
            None => (),
        }
    }

    // We want to study the function (t-k)k - d, search for = 0. Function develops to tk - k² - d, it goes like /\.
    // Derive to t - 2k, equals 0 at k = t/2, growing before and decreasing after.

    let kmid: u64 = time / 2;
    let kmin: u64 = 0;
    let kmax: u64 = time;

    let zero_left = binary_search(kmin, kmid, record_distance, |x| (time - x) * x);
    let zero_right = binary_search(kmax, kmid, record_distance, |x| (time - x) * x);

    println!("{}", (zero_right - 1) - (zero_left + 1) + 1);
}
