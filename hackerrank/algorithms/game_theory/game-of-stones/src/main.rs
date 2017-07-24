use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let t = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    for _ in 0..t {
        let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
        if n % 7 == 1 || n % 7 == 0 {
            println!("Second");
        } else {
            println!("First");
        }
    }
}
