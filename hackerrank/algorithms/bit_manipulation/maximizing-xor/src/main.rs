use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let l = lines.next().unwrap().trim().parse::<u32>().ok().unwrap();
    let r = lines.next().unwrap().trim().parse::<u32>().ok().unwrap();

    let ret = (l..r + 1)
        .flat_map(|i| (l..r + 1).map(move |j| i ^ j))
        .max()
        .unwrap();

    println!("{}", ret);
}
