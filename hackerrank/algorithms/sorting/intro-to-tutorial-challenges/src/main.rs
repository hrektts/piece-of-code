use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let v = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    let ar = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    println!("{}", ar.iter().position(|&x| x == v).unwrap());
}
