use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let mut ls = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    ls.sort_by(|a, b| b.cmp(a));

    for w in ls.windows(3) {
        if w[2] + w[1] > w[0] {
            println!("{} {} {}", w[2], w[1], w[0]);
            return;
        }
    }

    println!("-1");
}
