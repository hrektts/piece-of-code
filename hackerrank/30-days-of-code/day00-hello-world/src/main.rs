use std::io;

fn main() {
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .ok()
        .expect("read_error");

    println!("Hello, World.");
    println!("{}", input_str);
}
