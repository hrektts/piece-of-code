use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let q = lines.next().unwrap().trim().parse::<u32>().ok().unwrap();

    for _ in 0..q {
        let (l, r) = {
            let lr = lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<usize>().ok().unwrap())
                .collect::<Vec<_>>();
            (lr[0], lr[1])
        };

        let f = |x: usize| -> usize {
            match x % 8 {
                0...1 => x,
                2...3 => 2,
                4...5 => x + 2,
                _ => 0,
            }
        };

        println!("{}", f(r) ^ f(l - 1));
    }
}
