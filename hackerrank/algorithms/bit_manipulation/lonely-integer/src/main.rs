use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let ans = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u32>().ok().unwrap())
        .fold(0u32, |acc, x| acc ^ x);

    println!("{}", ans);
}
