use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let xs = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();
    let ws = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();
    let num = xs.iter()
        .zip(ws.iter())
        .fold(0f64, |acc, (&x, &w)| acc + x as f64 * w as f64);
    let den = ws.iter().fold(0f64, |acc, &w| acc + w as f64);

    println!("{:.1}", num / den);
}
