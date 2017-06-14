use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let nk = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let mut xs = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    xs.sort();

    let ans = if xs.last().unwrap() - xs.first().unwrap() <= 2 * nk[1] {
        1
    } else {
        xs.push(-1);
        let mut pos: Option<i32> = None;
        let mut anchor = *xs.first().unwrap();
        let mut ans = 0;
        for w in xs[..].windows(3) {
            if let Some(x) = pos {
                if w[0] - x > nk[1] {
                    anchor = w[0];
                    pos = None;
                } else if w[1] - x > nk[1] {
                    anchor = w[1];
                    pos = None;
                }
            }
            if pos.is_none() {
                if w[1] - anchor > nk[1] {
                    pos = Some(w[0]);
                    ans += 1;
                    if w[1] - w[0] > nk[1] && w[2] == -1 {
                        ans += 1;
                    }
                } else if w[2] == -1 {
                    ans += 1;
                }
            }
        }
        ans
    };

    println!("{}", ans);
}
