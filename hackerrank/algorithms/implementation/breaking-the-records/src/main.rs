use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let ss = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    let head = *ss.first().unwrap();
    let r = ss.iter().fold(
        (head, head, 0u32, 0u32),
        |acc, &x| if x > acc.0 {
            (x, acc.1, acc.2 + 1, acc.3)
        } else if x < acc.1 {
            (acc.0, x, acc.2, acc.3 + 1)
        } else {
            acc
        },
    );

    println!("{} {}", r.2, r.3);
}
