use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let d = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .ok()
        .unwrap();

    let xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<f64>().ok().unwrap())
        .collect::<Vec<_>>();

    let (left, right) = xs[..].split_at(d);
    for x in right.iter().chain(left.iter()) {
        print!("{} ", x);
    }
}
