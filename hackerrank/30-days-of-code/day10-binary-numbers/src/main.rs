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

    fn calc(acc: u64, max: u64, n: u64) -> u64 {
        let (acc, max) = if n % 2 == 1 {
            (acc + 1, max)
        } else {
            if acc > max { (0, acc) } else { (0, max) }
        };
        match n {
            0 => max,
            n => calc(acc, max, n / 2),
        }
    }

    println!("{}", calc(0, 0, n));
}
