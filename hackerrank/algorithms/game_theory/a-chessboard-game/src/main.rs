use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let (x, y) = {
            let xy = lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<usize>().ok().unwrap())
                .collect::<Vec<_>>();
            (xy[0] % 4, xy[1] % 4)
        };

        let ans = if y == 0 || y == 3 || x == 0 || x == 3 {
            "First"
        } else {
            "Second"
        };

        println!("{}", ans);
    }
}
