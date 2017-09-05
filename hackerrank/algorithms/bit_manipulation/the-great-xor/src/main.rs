use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let q = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..q {
        let x = lines.next().unwrap().trim().parse::<u64>().ok().unwrap();
        println!("{}", 2u64.pow(64 - x.leading_zeros()) - x - 1);
    }
}
