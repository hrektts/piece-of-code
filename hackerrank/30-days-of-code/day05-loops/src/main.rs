use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<u32>()
        .ok()
        .expect("parse error");

    for i in 1..11 {
        println!("{} x {} = {}", n, i, n * i);
    }
}
