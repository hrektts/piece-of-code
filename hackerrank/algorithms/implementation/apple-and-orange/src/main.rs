use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let st = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let ab = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let apples = lines
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let oranges = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i32>().ok().unwrap())
        .collect::<Vec<_>>();

    let num = apples
        .iter()
        .map(|x| ab[0] + x)
        .filter(|&x| x >= st[0] && x <= st[1])
        .count();
    println!("{}", num);

    let num = oranges
        .iter()
        .map(|x| ab[1] + x)
        .filter(|&x| x >= st[0] && x <= st[1])
        .count();
    println!("{}", num);
}
