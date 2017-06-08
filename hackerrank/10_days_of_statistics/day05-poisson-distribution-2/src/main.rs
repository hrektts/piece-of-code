use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let ls = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .take(2)
        .map(|s| s.parse::<f64>().ok().unwrap())
        .collect::<Vec<_>>();

    println!("{:.3}", 160. + 40. * (ls[0] + ls[0].powi(2)));
    println!("{:.3}", 128. + 40. * (ls[1] + ls[1].powi(2)));
}
