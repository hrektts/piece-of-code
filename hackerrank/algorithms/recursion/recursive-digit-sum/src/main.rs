extern crate num;

use std::io::{self, BufRead};
use num::{BigInt, Integer, Zero};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let (n, k) = {
        let mut iter = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| BigInt::parse_bytes(s.as_bytes(), 10).unwrap())
            .collect::<Vec<_>>()
            .into_iter();
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let nine = BigInt::parse_bytes(b"9", 10).unwrap();
    let x = n.checked_mul(&k).unwrap().mod_floor(&nine);

    println!("{}", if !x.is_zero() { x } else { nine });
}
