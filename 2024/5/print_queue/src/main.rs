use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut rules = Vec::new();
    let mut in_rules = true;
    let mut sum = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            in_rules = false;
            continue;
        }

        if in_rules {
            if let Some((l, r)) = line.split_once('|') {
                let left = l.parse::<u32>().unwrap();
                let right = r.parse::<u32>().unwrap();
                rules.push((left, right));
            }
        } else {
            let pages: Vec<u32> = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            if in_order(&pages, &rules) {
                // Always an odd number, so div by two gives the middle item when zero based.
                sum += pages[pages.len() / 2];
            }
        }
    }

    println!("Sum of middle pages is {}", sum);
}

fn in_order(pages: &[u32], rules: &[(u32, u32)]) -> bool {
    for (left, right) in rules {
        let mut seen_right = false;
        for p in pages {
            if p == right {
                seen_right = true;
            }

            if p == left && seen_right {
                return false;
            }
        }
    }
    true
}
