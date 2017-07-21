use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let (n, mut k) = {
        let nk = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();
        (nk[0], nk[1])
    };

    let mut lts = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<i64>().ok().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    lts.sort_by(|a, b| b[0].cmp(&a[0]));

    let ans = lts.iter().fold(0i64, |acc, x| if x[1] == 1 {
        if k <= 0 {
            acc - x[0]
        } else {
            k -= 1;
            acc + x[0]
        }
    } else {
        acc + x[0]
    });

    println!("{}", ans);
}
