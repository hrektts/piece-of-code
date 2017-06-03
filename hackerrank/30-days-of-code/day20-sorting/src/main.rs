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
    let mut array = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().expect("parse error"))
        .collect::<Vec<_>>();
    let mut cnt = 0;

    for _ in 0..n {
        let mut num_swap = 0;
        for j in 0..n - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                num_swap += 1;
            }
        }
        cnt += num_swap;
        if num_swap == 0 {
            break;
        }
    }

    println!("Array is sorted in {} swaps.", cnt);
    println!("First Element: {}", array[0]);
    println!("Last Element: {}", array[n - 1]);
}
