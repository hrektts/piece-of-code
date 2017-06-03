use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();

    io::stdin()
        .read_line(&mut a_str)
        .ok()
        .expect("read error");
    io::stdin()
        .read_line(&mut b_str)
        .ok()
        .expect("read error");

    let a: Vec<i32> = a_str
        .trim()
        .split(' ')
        .map(|v| v.parse().ok().expect("parse error"))
        .collect();
    let b: Vec<i32> = b_str
        .trim()
        .split(' ')
        .map(|v| v.parse().ok().expect("parse error"))
        .collect();

    let points = a.iter()
        .zip(b.iter())
        .fold((0, 0), |acc, (&av, &bv)| if av > bv {
            (acc.0 + 1, acc.1)
        } else if av < bv {
            (acc.0, acc.1 + 1)
        } else {
            acc
        });

    println!("{} {}", points.0, points.1);
}
