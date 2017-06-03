use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .ok()
        .expect("parse error");

    for _ in 0..t {
        let n_k = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().expect("parse error"))
            .collect::<Vec<_>>();

        if n_k[1] - 1 | n_k[1] <= n_k[0] {
            println!("{}", n_k[1] - 1);
        } else {
            println!("{}", n_k[1] - 2);
        }
    }
}
