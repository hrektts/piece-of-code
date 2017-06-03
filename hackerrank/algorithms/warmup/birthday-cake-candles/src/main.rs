use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .ok()
        .expect("parse error");
    let heights = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|v| v.parse::<u64>().ok().expect("parse error"))
        .take(n)
        .collect::<Vec<_>>();
    let max = heights.iter().max().unwrap();

    println!("{}",
             heights
                 .iter()
                 .filter(|&v| v == max)
                 .collect::<Vec<_>>()
                 .len());
}
