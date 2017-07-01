use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let map = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<String>().ok().unwrap())
        .fold(HashMap::new(), |mut acc, s| {
            *acc.entry(s).or_insert(0i32) += 1;
            acc
        });

    let map = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<String>().ok().unwrap())
        .fold(map, |mut acc, s| {
            *acc.entry(s).or_insert(0i32) -= 1;
            acc
        });

    let ans = if map.values().all(|&x| x >= 0) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
