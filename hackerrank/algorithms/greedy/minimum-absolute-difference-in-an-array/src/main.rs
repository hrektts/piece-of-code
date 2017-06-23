use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let mut xs = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().ok().unwrap())
        .collect::<Vec<_>>();

    xs.sort();

    let ans = xs.windows(2).map(|pair| pair[1] - pair[0]).min().unwrap();

    println!("{}", ans);
}
