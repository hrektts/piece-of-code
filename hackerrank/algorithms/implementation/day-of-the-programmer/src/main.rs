use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let y = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    if y == 1918 {
        println!("26.09.1918");
    } else if (y < 1918 && y % 4 == 0) || (y % 400 == 0 || y % 4 == 0 && y % 100 != 0) {
        println!("12.09.{}", y);
    } else {
        println!("13.09.{}", y);
    }
}
