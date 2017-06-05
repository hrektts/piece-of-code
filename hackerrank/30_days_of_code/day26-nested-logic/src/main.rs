use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let act = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().expect("parse error"))
        .collect::<Vec<_>>();

    let exp = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().expect("parse error"))
        .collect::<Vec<_>>();

    if act[2] > exp[2] {
        println!("10000");
        return;
    }

    if act[2] == exp[2] && act[1] > exp[1] {
        println!("{}", (act[1] - exp[1]) * 500);
        return;
    }

    if act[2] == exp[2] && act[1] == act[1] && act[0] > exp[0] {
        println!("{}", (act[0] - exp[0]) * 15);
        return;
    }

    println!("0");
}
