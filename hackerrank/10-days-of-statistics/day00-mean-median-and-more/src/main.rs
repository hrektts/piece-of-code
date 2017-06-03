use std::io::{self, BufRead};
use std::collections::HashMap;

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

    let mean = xs.iter().fold(0i32, |acc, &x| acc + x) as f64 / n as f64;
    println!("{}", mean);

    let median = if n % 2 == 0 {
        let idx = n / 2;
        (xs[idx - 1] + xs[idx]) as f64 / 2.
    } else {
        xs[n / 2 + 1] as f64
    };
    println!("{}", median);

    let mode = {
        let mut hist = HashMap::<i32, i32>::with_capacity(n);
        for x in xs {
            if hist.contains_key(&x) {
                *hist.get_mut(&x).unwrap() += 1;
            } else {
                hist.insert(x, 1);
            }
        }
        let mut hist = hist.drain().collect::<Vec<(i32, i32)>>();
        hist.sort_by(|a, b| if a.1 != b.1 {
                         b.1.cmp(&a.1)
                     } else {
                         a.0.cmp(&b.0)
                     });
        hist[0].0
    };
    println!("{}", mode);
}
