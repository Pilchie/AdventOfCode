use std::{collections::{HashSet, VecDeque}, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut towels: Vec<&str> = contents.lines().nth(0).unwrap().split(", ").collect();
    towels.sort_by(|t1, t2| t1.len().cmp(&t2.len()).reverse());
    let mut patterns = Vec::new();
    for line in contents.lines().skip(2) {
        patterns.push(line);
    }

    let mut sum = 0;
    for pattern in patterns {
        println!("Checking {}... ", pattern);
        let ways = ways_to_make(pattern, &towels);
        println!("{} ways.", ways);
        sum += ways;
    }

    println!("{} ways to make the patterns.", sum);
}

fn ways_to_make(pattern: &str, towels: &[&str]) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(vec![&pattern[0..0]]);
    let mut solutions = 0;

    while !queue.is_empty() {
        let curvec = queue.pop_back().unwrap();
        let curpath = curvec.join(",");
        let cur = curvec.join("");
        if cur == pattern {
            //println!("  possible.");
            solutions += 1;
        } else if seen.contains(&curpath) {
            continue;
        } else if cur.len() > pattern.len() {
            continue;
        }

        seen.insert(curpath.clone());

        let rest = &pattern[cur.len()..];
        //println!("\tcur is '{}', rest is '{}', curpath is {}", cur, rest, curpath);
        //print!("Considering: ");
        for t in towels {
            //print!("'{}' ", t);
            if rest.starts_with(t) {
                //print!("match, ");
                let mut next = curvec.clone();
                next.push(t);
                queue.push_back(next);
            } else {
                //print!("no match, ")
            }
        }
        //println!();
    }

    //println!("  NOT possible.");    
    solutions
}
