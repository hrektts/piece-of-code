use std::io::{self, BufRead};
use std::cmp::{Ordering, Ord, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Reverse(usize);

impl Ord for Reverse {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Reverse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Heap {
    low: BinaryHeap<usize>,
    high: BinaryHeap<Reverse>,
}

impl Heap {
    fn new() -> Self {
        Self {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }

    fn push(&mut self, v: usize) {
        if self.low.len() <= self.high.len() {
            self.low.push(v);
        } else {
            self.high.push(Reverse(v));
        }
        self.balance();
    }

    fn balance(&mut self) {
        while !self.low.is_empty() && !self.high.is_empty() &&
            *self.low.peek().unwrap() > self.high.peek().unwrap().0
        {
            let l = self.low.pop().unwrap();
            let h = self.high.pop().unwrap().0;
            self.low.push(h);
            self.high.push(Reverse(l));
        }
    }

    fn median(&mut self) -> Option<f64> {
        if self.low.is_empty() {
            None
        } else if self.low.len() == self.high.len() {
            Some(
                (self.low.peek().unwrap() + self.high.peek().unwrap().0) as f64 / 2.0,
            )
        } else {
            Some(*self.low.peek().unwrap() as f64)
        }
    }
}

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
    let mut heap = Heap::new();
    for _ in 0..n {
        let a = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();
        heap.push(a);
        if let Some(median) = heap.median() {
            println!("{:.1}", median);
        }
    }
}
