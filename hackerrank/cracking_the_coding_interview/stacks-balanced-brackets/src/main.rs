use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let n = lines.next().unwrap().trim().parse::<usize>().ok().unwrap();

    let anss = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .trim()
                .parse::<String>()
                .ok()
                .unwrap()
                .chars()
                .fold(Some(vec![]), |acc, x| if let Some(mut acc) = acc {
                    match x {
                        c if c == '(' || c == '{' || c == '[' => {
                            acc.push(c);
                            Some(acc)
                        }
                        c => {
                            match acc.pop() {
                                Some(cc) if c == ')' && cc == '(' => Some(acc),
                                Some(cc) if c == '}' && cc == '{' => Some(acc),
                                Some(cc) if c == ']' && cc == '[' => Some(acc),
                                _ => None,
                            }
                        }
                    }
                } else {
                    None
                })
                .and_then(|v| if v.len() == 0 { Some(()) } else { None })
        })
        .collect::<Vec<_>>();

    for ans in anss {
        let out = if ans.is_some() { "YES" } else { "NO" };
        println!("{}", out);
    }
}
