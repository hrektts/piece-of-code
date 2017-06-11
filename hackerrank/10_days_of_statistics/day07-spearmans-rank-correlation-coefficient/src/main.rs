use std::io::{self, BufRead};
use std::cmp::Ordering;

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

    let xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<f64>().ok().unwrap())
        .collect::<Vec<_>>();

    let ys = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<f64>().ok().unwrap())
        .collect::<Vec<_>>();

    let mut xs_sorted = xs.clone();
    xs_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    let rank_x = xs.iter()
        .map(|&x| {
                 xs_sorted
                     .iter()
                     .position(|&x_sorted| x_sorted == x)
                     .unwrap() as f64 + 1.
             })
        .collect::<Vec<_>>();

    let mut ys_sorted = ys.clone();
    ys_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    let rank_y = ys.iter()
        .map(|&y| {
                 ys_sorted
                     .iter()
                     .position(|&y_sorted| y_sorted == y)
                     .unwrap() as f64 + 1.
             })
        .collect::<Vec<_>>();

    let r_x_y = 1. -
                6. *
                rank_x
                    .iter()
                    .zip(rank_y.iter())
                    .fold(0f64, |acc, (&rx, &ry)| acc + (rx - ry).powi(2)) /
                (n * (n * n - 1)) as f64;

    println!("{:.3}", r_x_y);
}
