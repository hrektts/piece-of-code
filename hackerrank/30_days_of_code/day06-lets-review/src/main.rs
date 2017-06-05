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
    for _ in 0..n {
        let s = lines
            .next()
            .unwrap()
            .trim()
            .parse::<String>()
            .ok()
            .expect("parse error");
        let even = s.chars()
            .enumerate()
            .filter(|t| (t.0 % 2) == 0)
            .map(|t| t.1)
            .collect::<String>();
        let odd = s.chars()
            .enumerate()
            .filter(|t| (t.0 % 2) == 1)
            .map(|t| t.1)
            .collect::<String>();
        println!("{} {}", even, odd);
    }
}
