use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut rules = Vec::new();
    let mut in_rules = true;
    let mut sum_correct = 0;
    let mut sum_fixed = 0;
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
                sum_correct += pages[pages.len() / 2];
            } else {
                let fixed = fix_order(&pages, &rules);
                sum_fixed += fixed[fixed.len() / 2];
            }
        }
    }

    println!(
        "Sum of middle pages for already correct updates is {}",
        sum_correct
    );
    println!("Sum of middle pages for fixed updates is {}", sum_fixed);
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

fn fix_order(pages: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    //println!("trying to fix  {:?}", pages);
    let mut pages = pages.to_vec();
    let mut new_rules:Vec<_> = rules.into_iter().filter(|(l, r)| pages.contains(l) && pages.contains(r)).map(|r| r.clone()).collect();
    let mut new_pages = Vec::new();

    while pages.len() > 0 {
        if let Some(next) = find_next(&pages, &new_rules){
            new_pages.push(next);
            let index = pages.iter().position(|p| *p == next).unwrap();
            pages.remove(index);

            let mut to_remove = Vec::new();
            for i in 0..new_rules.len() {
                if new_rules[i].0 == next || new_rules[i].1 == next {
                    to_remove.push(i);
                }
            }
            to_remove.reverse();

            for i in to_remove {
                new_rules.remove(i);
            }
        } else {
            panic!("Didn't find another number to pick");
        }
    }

    //println!("fixed order is {:?}", new_pages);
    new_pages
}

fn find_next(pages: &[u32], rules: &[(u32, u32)]) -> Option<u32> {
    for p in pages {
        let mut on_right = false;
        for (_, r) in rules {
            if r == p {
                on_right = true;
                break;
            }
        }

        if !on_right {
            return Some(*p);
        }
    }

    None
}
