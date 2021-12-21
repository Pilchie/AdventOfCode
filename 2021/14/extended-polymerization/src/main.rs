use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let steps = args[2].parse::<usize>().unwrap();

    let print = false;

    let template: Vec<u8> = input.lines().next().unwrap().bytes().collect();

    let mut rule_counts = HashMap::new();
    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        match line.split_once(" -> ") {
            Some((l, r)) => {
                let lc1 = l.bytes().nth(0).unwrap();
                let lc2 = l.bytes().nth(1).unwrap();
                let rc = r.bytes().nth(0).unwrap();
                rules.insert((lc1, lc2), rc);
                (*rule_counts.entry(lc1).or_insert(0)) += 1;
                (*rule_counts.entry(lc2).or_insert(0)) += 1;
                (*rule_counts.entry(rc).or_insert(0)) += 1;

            },
            None => panic!("Unexpected line: '{}'", line),
        };
    }

    let mut chars : Vec<&u8> = rule_counts.keys().collect();
    chars.sort();

    println!("Occurrences in rules: ");
    for (k, v) in &rule_counts {
        println!("{}: {}", *k as char, v);
    }

    let mut chain = step(&template, &rules);
    let mut histogram = build_histogram(&chain);
    for s in 1..steps {
        chain = step(&chain, &rules);

        let new = build_histogram(&chain);
        for c in &chars {
            let prevcount = histogram[c];
            let newcount = new[c];
            let ratio = newcount as f64 / prevcount as f64;
            print!("Step {} [{}: count: {:9}, ratio: {:.4}] - ", s+1, **c as char, newcount, ratio);
        }
        println!();

        histogram = new;
        if print {
            print_chain(s + 1, &chain);
        }
    }

    for s in steps+1..41 {
        println!("multiplying {}", s);
        for c in &chars {
            (*histogram.entry(**c).or_insert(0)) *= 2;
        }
    }

    let mut maxcount = 0u128;
    let mut maxchar = 0u8;
    let mut mincount = u128::MAX;
    let mut minchar = 0u8;

    for (k, v) in histogram {
        if v > maxcount {
            maxcount = v;
            maxchar = k;
        } else if v < mincount {
            mincount = v;
            minchar = k;
        }
    }

    println!(
        "Min char is {} with {}, max is {} with {}.  Difference is {}",
        minchar as char,
        mincount,
        maxchar as char,
        maxcount,
        maxcount - mincount,
    );

    Ok(())
}

fn step(input: &Vec<u8>, rules: &HashMap<(u8, u8), u8>) -> Vec<u8> {
    let mut res = Vec::new();
    res.reserve(input.len() * 2);

    for i in 0..input.len() - 1 {
        let l = input[i];
        let r = input[i + 1];

        let c = match rules.get(&(l, r)) {
            Some(c) => c,
            None => panic!("No rule for {}{}", l, r),
        };

        res.push(l);
        res.push(*c);
    }

    res.push(*input.last().unwrap());
    res
}

fn print_chain(s: usize, chain: &Vec<u8>) {
    print!("After step {}: ", s);
    for c in chain {
        print!("{}", *c as char);
    }
    println!();
}

fn build_histogram(chain: &Vec<u8>) -> HashMap<u8, u128> {
    let mut histogram = HashMap::new();
    for c in chain {
        (*histogram.entry(*c).or_insert(0)) += 1;
    }

    histogram
}