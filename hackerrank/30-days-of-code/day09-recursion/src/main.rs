use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<u64>()
        .ok()
        .expect("parse error");

    fn factorial(n: u64) -> u64 {
        match n {
            n if n <= 1 => 1,
            n => n * factorial(n - 1),
        }
    }

    println!("{}", factorial(n));
}
