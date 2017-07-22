use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let k = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>()
        [1];

    let cs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let b = lines.next().unwrap().trim().parse::<i32>().ok().unwrap();

    let even = cs.iter().enumerate().fold(0i32, |acc, (i, &x)| if i != k {
        acc + x
    } else {
        acc
    }) / 2;

    match b - even {
        0 => println!("Bon Appetit"),
        v => println!("{}", v),
    }
}
