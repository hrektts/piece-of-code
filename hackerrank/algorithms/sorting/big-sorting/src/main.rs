use std::io::{self, BufRead};
use std::cmp::Ord;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .ok()
        .unwrap();

    let mut nums = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .parse::<String>()
                .ok()
                .unwrap()
        })
        .collect::<Vec<_>>();

    nums.sort_by(|a, b| {
                     let la = a.len();
                     let lb = b.len();
                     if la != lb { la.cmp(&lb) } else { a.cmp(&b) }
                 });

    for num in nums {
        println!("{}", num);
    }

}
