use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let size = lines
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .ok()
        .expect("parse error");

    for _ in 0..size {
        let v = lines
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .ok()
            .expect("parse error");

        if v == 2 {
            println!("Prime");
            continue;
        } else if v < 2 || v % 2 == 0 {
            println!("Not prime");
            continue;
        } else {
            let mut n = 3;
            while n as f64 <= (v as f64).sqrt() {
                if v % n == 0 {
                    println!("Not prime");
                    break;
                }
                n += 2;
            }
            if v % n != 0 {
                println!("Prime");
            }
        }
    }
}
