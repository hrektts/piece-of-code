use std::io::{self, BufRead};
use std::str;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let v = lines
        .next()
        .unwrap()
        .trim()
        .split(':')
        .map(|v| v.parse::<String>().ok().expect("parse error"))
        .collect::<Vec<_>>();

    match &v[2][2..4] {
        "PM" if v[0] == "12" => println!("{}:{}:{}", v[0], v[1], &v[2][0..2]),
        "PM" => {
            let h = v[0].parse::<u32>().ok().expect("parse error");
            println!("{}:{}:{}", h + 12, v[1], &v[2][0..2]);
        }
        "AM" if v[0] == "12" => println!("00:{}:{}", v[1], &v[2][0..2]),
        _ => println!("{}:{}:{}", v[0], v[1], &v[2][0..2]),
    }
}
