use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let (_level, _mountain, valley) = lines
        .nth(1)
        .unwrap()
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .fold((0isize, 0isize, 0isize), |acc, c| match c {
            'U' => {
                if acc.0 == -1 {
                    (0, acc.1, acc.2 + 1)
                } else {
                    (acc.0 + 1, acc.1, acc.2)
                }
            }
            'D' => {
                if acc.0 == 1 {
                    (0, acc.1 + 1, acc.2)
                } else {
                    (acc.0 - 1, acc.1, acc.2)
                }
            }
            _ => unreachable!(),
        });

    println!("{}", valley);
}
