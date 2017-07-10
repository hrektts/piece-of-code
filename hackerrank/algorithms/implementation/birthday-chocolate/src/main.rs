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

    let (d, m) = {
        let dm = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();
        (dm[0], dm[1])
    };

    let n = ss.windows(m)
        .filter(|&w| w.iter().sum::<usize>() == d)
        .count();

    println!("{}", n);
}
