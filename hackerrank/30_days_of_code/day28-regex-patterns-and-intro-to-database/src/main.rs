use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .ok()
        .expect("parse error");

    let mut xs = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.parse::<String>().ok().expect("parse error"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
        .iter()
        .filter_map(|ss| if ss[1].ends_with("@gmail.com") {
                        Some(ss[0].clone())
                    } else {
                        None
                    })
        .collect::<Vec<_>>();

    xs.sort();

    for x in xs {
        println!("{}", x);

    }
}
