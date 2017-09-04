use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines.next().unwrap().trim().parse::<u64>().ok().unwrap();

    println!("{}", 2u64.pow(n.count_zeros() - n.leading_zeros()));
}
