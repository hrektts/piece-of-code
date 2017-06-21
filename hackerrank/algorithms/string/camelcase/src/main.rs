use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .filter(|c| c.is_uppercase())
        .count();

    println!("{}", n + 1);
}
