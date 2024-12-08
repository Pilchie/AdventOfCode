use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let mut sum = 0;
    for line in contents.lines() {
        println!("starting line {}", line);
        if let Some((totalstr, rest)) = line.split_once(": ") {
            let total = totalstr.parse::<u64>().unwrap();
            let mut terms: Vec<_> = rest.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            // We need to process terms right to left, but because we are recursing, we'll handle the last first,
            // so revert it.
            terms.reverse();

            if possible(total, &terms) {
                sum += total;
            }
        }
    }

    println!("Sum is {}", sum);
}

fn possible(total: u64, terms: &[u64]) -> bool {
    let combinations = collect_combinations(terms);
    for c in combinations {
        if c == total {
            println!("Found a valid combo!");
            return true;
        }
    }

    false
}

fn collect_combinations(terms: &[u64]) -> Vec<u64> {
    let mut res = Vec::new();

    if terms.len() == 1 { 
        res.push(terms[0]);
        return res;
    }
    
    for c in collect_combinations(&terms[1..]) {
        res.push(terms[0] + c);
        res.push(terms[0] * c);
        let l = format!("{}", c);
        let r = format!("{}", terms[0]);
        let new = (l + &r).parse::<u64>().unwrap();
        res.push(new);
    }

    res
}
