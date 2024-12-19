use std::{collections::{HashSet, VecDeque}, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let towels: Vec<&str> = contents.lines().nth(0).unwrap().split(", ").collect();
    let mut patterns = Vec::new();
    for line in contents.lines().skip(2) {
        patterns.push(line);
    }

    let mut count = 0;
    for pattern in patterns {
        if can_make(pattern, &towels) {
            count += 1;
        }
    }

    println!("{} patterns are possible", count);
}

fn can_make(pattern: &str, towels: &[&str]) -> bool {
    println!("Checking {}... ", pattern);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(&pattern[0..0]);

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if cur == pattern {
            println!("  possible.");
            return true;
        } else if seen.contains(cur) {
            continue;
        }
        seen.insert(cur);

        let rest = &pattern[cur.len()..];
        //println!("\tcur is '{}', rest is '{}'", cur, rest);
        //print!("Considering: ");
        for t in towels {
            //print!("'{}' ", t);
            if rest.starts_with(t) {
                //print!("match, ");
                let next = &pattern[0..cur.len() + t.len()];
                queue.push_back(next);
            } else {
                //print!("no match, ")
            }
        }
        //println!();
    }
    println!("  NOT possible.");
    false
}
