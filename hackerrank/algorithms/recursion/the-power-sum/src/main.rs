use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let x = lines.next().unwrap().trim().parse::<i64>().ok().unwrap();
    let n = lines.next().unwrap().trim().parse::<u32>().ok().unwrap();

    fn solve(x: i64, n: u32, num: u32) -> u32 {
        match x - num.pow(n) as i64 {
            v if v < 0 => 0,
            0 => 1,
            v => solve(v, n, num + 1) + solve(x, n, num + 1),
        }
    }

    println!("{}", solve(x, n, 1));
}
