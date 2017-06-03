use std::io;

fn main() {
    let mut num_element_str = String::new();

    io::stdin()
        .read_line(&mut num_element_str)
        .ok()
        .expect("read error");

    let num_element: i32 = num_element_str
        .trim()
        .parse()
        .ok()
        .expect("parse error");

    let pair =
        (0..num_element)
            .map(|_| {
                let mut elements_str = String::new();
                io::stdin()
                    .read_line(&mut elements_str)
                    .ok()
                    .expect("read error");
                let row: Vec<i32> = elements_str
                    .trim()
                    .split(' ')
                    .map(|v| v.parse().ok().expect("parse error"))
                    .collect();
                row
            })
            .enumerate()
            .fold((0, 0),
                  |acc, (i, row)| (acc.0 + row[i], acc.1 + row[num_element as usize - i - 1]));

    println!("{}", (pair.0 - pair.1).abs());
}
