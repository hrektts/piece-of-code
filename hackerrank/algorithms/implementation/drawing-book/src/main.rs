use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    let p = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    println!("{}", cmp::min(p / 2, n / 2 - p / 2));
}
