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

    let p_rej = xs[0] as f64 / 100.;

    let p = |p: f64, n: i32, x: i32| -> f64 {
        combination(n, x) as f64 * p.powi(x) * (1. - p).powi(n - x)
    };

    let ans = (0..3)
        .map(|x| p(p_rej, xs[1], x))
        .fold(0f64, |acc, p| acc + p);

    println!("{:.3}", ans);

    let ans = (2..(xs[1] + 1))
        .map(|x| p(p_rej, xs[1], x))
        .fold(0f64, |acc, p| acc + p);

    println!("{:.3}", ans);
}

fn combination(n: i32, x: i32) -> i32 {
    ((n - x + 1)..(n + 1)).fold(1i32, |acc, v| acc * v) / (1..(x + 1)).fold(1i32, |acc, v| acc * v)
}
