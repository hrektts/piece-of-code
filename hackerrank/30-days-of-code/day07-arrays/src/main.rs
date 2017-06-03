use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());
    let mut v = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|v| v.parse().ok().expect("parse error"))
        .collect::<Vec<u32>>();
    v.reverse();
    let n = v[1..]
        .iter()
        .fold(v[0].to_string(), |mut acc, &x| {
            acc.push_str(" ");
            acc.push_str(x.to_string().as_ref());
            acc
        });

    println!("{}", n);
}
