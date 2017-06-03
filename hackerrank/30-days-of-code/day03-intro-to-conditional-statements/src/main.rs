use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let v = lines
        .next()
        .unwrap()
        .trim()
        .parse::<u32>()
        .ok()
        .expect("parse error");

    if v == 0 {
        return;
    } else if v % 2 == 1 {
        println!("Weird");
    } else if v >= 2 && v <= 5 {
        println!("Not Weird");
    } else if v >= 6 && v <= 20 {
        println!("Weird");
    } else {
        println!("Not Weird");
    }
}
