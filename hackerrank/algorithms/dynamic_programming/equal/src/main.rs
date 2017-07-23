use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let mut xs = lines
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();

        xs.sort();

        let target = xs[0];
        let ans = (0..3)
            .map(|base| {
                xs.iter().fold(0usize, |acc, &x| {
                    let diff = x - target + base;
                    acc + diff / 5 + diff % 5 / 2 + diff % 5 % 2 / 1
                })
            })
            .min()
            .unwrap();

        println!("{}", ans);
    }
}
