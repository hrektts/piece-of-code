use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let map = lines
        .next()
        .unwrap()
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .fold(HashMap::with_capacity(26), |mut acc, c| {
            *acc.entry(c).or_insert(0i32) += 1;
            acc
        });

    let map = lines
        .next()
        .unwrap()
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .fold(map, |mut acc, c| {
            *acc.entry(c).or_insert(0i32) -= 1;
            acc
        });

    println!("{}", map.values().fold(0i32, |acc, &v| acc + v.abs()));
}
