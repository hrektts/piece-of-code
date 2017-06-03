use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let m = (0..6)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|v| v.parse().ok().expect("parse error"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let max = (0..4)
        .map(|u| {
            (0..4)
                .map(|v| {
                         m[u][v] + m[u][v + 1] + m[u][v + 2] + m[u + 1][v + 1] + m[u + 2][v] +
                         m[u + 2][v + 1] + m[u + 2][v + 2]
                     })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", max);
}
