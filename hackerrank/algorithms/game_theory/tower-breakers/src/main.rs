use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let (n, m) = {
            let nm = lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<usize>().ok().unwrap())
                .collect::<Vec<_>>();
            (nm[0], nm[1])
        };

        let ans = if m == 1 || n % 2 == 0 { 2 } else { 1 };
        println!("{}", ans);
    }
}
