use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let q = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    let mut queue = VecDeque::new();
    for _ in 0..q {
        let query = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            .collect::<Vec<_>>();

        match query[0] {
            1 => queue.push_back(query[1]),
            2 => {
                queue.pop_front();
            }
            3 => println!("{}", queue.front().unwrap()),
            _ => unreachable!(),
        }
    }
}
