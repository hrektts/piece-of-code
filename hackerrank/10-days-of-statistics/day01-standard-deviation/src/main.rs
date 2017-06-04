use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .ok()
        .unwrap();

    let xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let mean = xs.iter().fold(0i32, |acc, x| acc + x) as f64 / n as f64;
    let div = xs.iter()
        .fold(0f64, |acc, &x| {
            let diff = x as f64 - mean;
            acc + diff * diff
        }) / n as f64;

    println!("{:.1}", div.sqrt());
}
