use std::io::{self, BufRead};

fn main() {
    let io = io::stdin();
    let mut lines = io.lock().lines().filter_map(|s| s.ok());

    let s = lines
        .next()
        .unwrap()
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    fn reduce(mut ss: Vec<char>) -> Option<Vec<char>> {
        if ss.is_empty() {
            None
        } else {
            let before = ss.len();
            for i in 0..(before - 1) {
                if ss[i] == ss[i + 1] {
                    ss.remove(i);
                    ss.remove(i);
                    break;
                }
            }
            if ss.len() != before {
                reduce(ss)
            } else {
                Some(ss)
            }
        }
    }

    reduce(s)
        .and_then(|reduced| {
                      println!("{}", reduced.into_iter().collect::<String>());
                      Some(())
                  })
        .or_else(|| {
                     println!("Empty String");
                     Some(())
                 });
}
