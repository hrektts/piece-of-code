use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let (_, k) = {
        let nk = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();
        (nk[0], nk[1])
    };

    let mut nums = vec![0usize; k];
    let count = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .fold(0usize, |mut acc, x| {
            let m = x % k;
            acc += nums[(k - m) % k];
            nums[m] += 1;
            acc
        });

    println!("{}", count);
}
