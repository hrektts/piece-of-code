use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let v = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|v| v.parse::<u64>().ok().expect("parse error"))
        .collect::<Vec<_>>();
    let sum = (0..5)
        .map(|i| v.iter().sum::<u64>() - v[i])
        .collect::<Vec<_>>();

    println!("{} {}",
             sum.iter().min().unwrap(),
             sum.iter().max().unwrap());
}
