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

    for i in 0..size {
        let padding = String::from_utf8(vec![b' '; size - i - 1]).unwrap();
        let symbols = String::from_utf8(vec![b'#'; i + 1]).unwrap();
        println!("{}{}", padding, symbols);
    }
}
