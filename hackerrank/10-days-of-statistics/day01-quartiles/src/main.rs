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

    let mut xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    xs.sort();

    let first = median(&xs[..(n / 2)]).unwrap();
    let second = median(&xs[..]).unwrap();
    let third = if n % 2 == 0 {
        median(&xs[(n / 2)..]).unwrap()
    } else {
        median(&xs[(n / 2 + 1)..]).unwrap()
    };

    println!("{}\n{}\n{}", first, second, third);

}

fn median<T: Clone + Into<f64>>(xs: &[T]) -> Option<f64> {
    match xs.len() {
        0 => None,
        1 => Some(xs[0].clone().into()),
        len if len % 2 == 0 => {
            let v1: f64 = xs[(len / 2) - 1].clone().into();
            let v2: f64 = xs[len / 2].clone().into();
            Some((v1 + v2) / 2.)
        }
        len => Some(xs[len / 2].clone().into()),
    }
}
