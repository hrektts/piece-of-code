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

    let _num_element: i32 = num_element_str
        .trim()
        .parse()
        .ok()
        .expect("parse error");
    let elements: Vec<i32> = elements_str
        .trim()
        .split(' ')
        .map(|v| v.parse().ok().expect("parse error"))
        .collect();
    let sum: i32 = elements.iter().sum();

    println!("{}", sum);
}
