use std::io;

fn main() {
    let mut num_element_str = String::new();
    let mut elements_str = String::new();

    io::stdin()
        .read_line(&mut num_element_str)
        .ok()
        .expect("read error");
    io::stdin()
        .read_line(&mut elements_str)
        .ok()
        .expect("read error");

    let num_element: i32 = num_element_str
        .trim()
        .parse()
        .ok()
        .expect("parse error");
    let elements: Vec<i32> = elements_str
        .trim()
        .split(' ')
        .map(|v| v.parse().ok().expect("parse error"))
        .collect();
    let sum = elements
        .iter()
        .fold((0, 0, 0), |acc, &v| if v > 0 {
            (acc.0 + 1, acc.1, acc.2)
        } else if v < 0 {
            (acc.0, acc.1 + 1, acc.2)
        } else {
            (acc.0, acc.1, acc.2 + 1)
        });

    println!("{:.6}", sum.0 as f64 / num_element as f64);
    println!("{:.6}", sum.1 as f64 / num_element as f64);
    println!("{:.6}", sum.2 as f64 / num_element as f64);
}
