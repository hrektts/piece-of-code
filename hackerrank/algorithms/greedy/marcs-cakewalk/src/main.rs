use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let mut cs = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    cs.sort_by(|a, b| b.cmp(a));

    let ans = cs.iter().enumerate().fold(0usize, |acc, (i, x)| {
        acc + x * 2usize.pow(i as u32)
    });

    println!("{}", ans);
}
