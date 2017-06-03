use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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
    let mut phone_book = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|v| v.parse().ok().expect("parse error"))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .map(|record| (record[0].clone(), record[1].clone()))
        .collect::<HashMap<String, String>>();

    for _ in 0..n {
        let name = lines
            .next()
            .unwrap()
            .trim()
            .parse::<String>()
            .ok()
            .expect("parse error");
        match phone_book.entry(name) {
            Entry::Occupied(e) => println!("{}={}", e.key(), e.get()),
            Entry::Vacant(_) => println!("Not found"),
        }
    }
}
