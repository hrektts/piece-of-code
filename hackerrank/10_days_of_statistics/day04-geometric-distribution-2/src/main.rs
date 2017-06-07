use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .take(2)
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .ok()
        .unwrap();

    let p_d = xs[0] as f64 / xs[1] as f64;
    let ans = (1..(n + 1))
        .map(|i| (1. - p_d).powi(i - 1) * p_d)
        .fold(0f64, |acc, p| acc + p);

    println!("{:.3}", ans);
}
