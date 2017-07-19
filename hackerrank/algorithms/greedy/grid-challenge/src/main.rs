use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
        let g = (0..n)
            .map(|_| {
                let mut row = lines
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<String>()
                    .ok()
                    .unwrap()
                    .chars()
                    .collect::<Vec<_>>();
                row.sort();
                row
            })
            .collect::<Vec<_>>();

        let mut ans = "YES";
        for i in 0..n {
            for j in 0..n - 1 {
                if g[j][i] > g[j + 1][i] {
                    ans = "NO";
                }
            }
        }
        println!("{}", ans);
    }
}
