use std::io::{self, BufRead};

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

    let mu_x = xs.iter().sum::<f64>() / n as f64;
    let v_x = xs.iter().fold(0f64, |acc, &x| acc + (x - mu_x).powi(2)) / n as f64;
    let sigma_x = v_x.sqrt();
    let mu_y = ys.iter().sum::<f64>() / n as f64;
    let v_y = ys.iter().fold(0f64, |acc, &y| acc + (y - mu_y).powi(2)) / n as f64;
    let sigma_y = v_y.sqrt();
    let ro_x_y = xs.iter()
        .zip(ys.iter())
        .fold(0f64, |acc, (&x, &y)| acc + (x - mu_x) * (y - mu_y)) /
                 (n as f64 * sigma_x * sigma_y);

    println!("{:.3}", ro_x_y);
}
