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
        .unwrap();

    for _ in 0..n {
        let g = lines
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .ok()
            .unwrap();

        let rounded = match g {
            g if g < 38 => g,
            g if g % 5 > 2 => ((g + 5) / 5) * 5,
            g => g,
        };

        println!("{}", rounded);
    }
}
