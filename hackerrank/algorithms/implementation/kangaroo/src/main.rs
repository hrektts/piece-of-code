use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let (x1, v1, x2, v2) = {
        let vs = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<i32>().ok().unwrap())
            .collect::<Vec<_>>();
        (vs[0], vs[1], vs[2], vs[3])
    };

    let ans = if x1 < x2 && v1 <= v2 || x1 > x2 && v1 >= v2 {
        "NO"
    } else if (x1 - x2).abs() % (v1 - v2).abs() == 0 {
        "YES"
    } else {
        "NO"
    };

    println!("{}", ans);
}
