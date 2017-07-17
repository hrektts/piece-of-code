use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let m = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
        let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
        let cs = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();

        let mut pair = None;
        for i in 0..n {
            for j in i + 1..n {
                if cs[i] + cs[j] == m {
                    pair = if i < j {
                        Some((i + 1, j + 1))
                    } else {
                        Some((j + 1, i + 1))
                    };
                    break;
                }
            }
            if pair.is_some() {
                break;
            }
        }

        println!("{} {}", pair.unwrap().0, pair.unwrap().1);
    }
}
