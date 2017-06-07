use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let ps = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .take(2)
        .map(|s| s.parse::<f64>().ok().unwrap())
        .collect::<Vec<_>>();

    let p_boy = ps[0] / ps.iter().fold(0f64, |acc, &p| acc + p);

    let p = |p: f64, n: i32, x: i32| -> f64 {
        combination(n, x) as f64 * p.powi(x) * (1. - p).powi(n - x)
    };

    let ans = (3..7)
        .map(|x| p(p_boy, 6, x))
        .fold(0f64, |acc, p| acc + p);

    println!("{:.3}", ans);
}

fn combination(n: i32, x: i32) -> i32 {
    ((n - x + 1)..(n + 1)).fold(1i32, |acc, v| acc * v) / (1..(x + 1)).fold(1i32, |acc, v| acc * v)
}
