use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap().trim().parse::<u64>().ok().unwrap();
        let ans = if ((n - 1).count_ones() & 1) != 0 {
            "Louise"
        } else {
            "Richard"
        };
        println!("{}", ans);
    }
}
