use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let lambda = lines
        .next()
        .unwrap()
        .trim()
        .parse::<f64>()
        .ok()
        .unwrap();

    let k = lines
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .ok()
        .unwrap();

    let ans = lambda.powi(k) * (-lambda).exp() / (1..(k + 1)).fold(1i32, |acc, k| acc * k) as f64;

    println!("{:.3}", ans);
}
