use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let xs = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let fs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let mut s = Vec::new();
    for (x, f) in xs.iter().zip(fs.iter()) {
        for _ in 0..f.clone() {
            s.push(x.clone());
        }
    }

    s.sort();

    let len = s.len();
    let first = median(&s[..(len / 2)]).unwrap();
    let third = if len % 2 == 0 {
        median(&s[(len / 2)..]).unwrap()
    } else {
        median(&s[(len / 2 + 1)..]).unwrap()
    };

    println!("{:.1}", third - first);
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
