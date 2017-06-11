use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let score = (0..5)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<f64>().ok().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let (sum_xy, sum_x, sum_y, sum_x2) = score
        .iter()
        .fold((0f64, 0f64, 0f64, 0f64), |acc, xy| {
            (acc.0 + xy[0] * xy[1], acc.1 + xy[0], acc.2 + xy[1], acc.3 + xy[0] * xy[0])
        });

    let b = (5. * sum_xy - sum_x * sum_y) / (5. * sum_x2 - sum_x * sum_x);
    let a = sum_y / 5. - b * sum_x / 5.;

    println!("{:.3}", a + b * 80.);
}
