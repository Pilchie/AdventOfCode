use std::{collections::HashMap, env, fs};

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
        print!("Checking {}... ", pattern);
        let mut memoized_patterns = HashMap::new();
        let ways = ways_to_make(pattern, &towels, &mut memoized_patterns);
        println!("{} ways.", ways);
        sum += ways;
    }

    println!("{} ways to make the patterns.", sum);
}

fn ways_to_make<'a>(pattern: &'a str, towels: &[&str], seen: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(count) = seen.get(pattern) {
        return *count;
    } else {
        let mut solutions = 0;
        for t in towels {
            if *t == pattern {
                solutions += 1;
            } else if pattern.starts_with(t) {
                solutions += ways_to_make(&pattern[t.len()..], towels, seen)
            }
        }

        seen.insert(pattern, solutions);
        solutions
    }
}
