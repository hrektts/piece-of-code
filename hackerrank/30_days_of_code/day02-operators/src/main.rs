use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let meal_cost = lines
        .next()
        .unwrap()
        .trim()
        .parse::<f64>()
        .ok()
        .expect("parse error");
    let tip_percent = lines
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .ok()
        .expect("parse error");
    let tax_percent = lines
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .ok()
        .expect("parse error");

    let total = (meal_cost * (100 + tip_percent + tax_percent) as f64 / 100.0).round();

    println!("The total meal cost is {:.0} dollars.", total);
}
