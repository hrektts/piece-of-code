extern crate num;

use std::io::{self, BufRead};
use num::integer::{gcd, lcm};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let aa = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().ok().unwrap())
        .collect::<Vec<_>>();

    let bb = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().ok().unwrap())
        .collect::<Vec<_>>();

    let lcm_a = aa.iter().fold(1i64, |acc, &a| lcm(acc, a));
    let gcd_b = bb.iter().fold(0i64, |acc, &b| gcd(acc, b));

    let ans = (lcm_a..(gcd_b + 1))
        .filter(|n| n % lcm_a == 0)
        .filter(|n| gcd_b % n == 0)
        .count();

    println!("{}", ans);
}
