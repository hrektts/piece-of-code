use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    let mut aa = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    let mut ans = 0usize;
    for _ in 0..n {
        let mut number_of_swaps = 0usize;
        for j in 0..n - 1 {
            if aa[j] > aa[j + 1] {
                aa.swap(j, j + 1);
                number_of_swaps += 1;
            }
        }

        if number_of_swaps == 0 {
            break;
        } else {
            ans += number_of_swaps;
        }
    }

    println!("Array is sorted in {} swaps.", ans);
    println!("First Element: {}", aa[0]);
    println!("Last Element: {}", aa[n - 1]);
}
