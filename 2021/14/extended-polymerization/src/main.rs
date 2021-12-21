use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let print = false;

    let template: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        match line.split_once(" -> ") {
            Some((l, r)) => rules.insert(
                (l.chars().nth(0).unwrap(), l.chars().nth(1).unwrap()),
                r.chars().nth(0).unwrap(),
            ),
            None => panic!("Unexpected line: '{}'", line),
        };
    }

    let mut chain = template;
    for s in 0..10 {
        chain = step(&chain, &rules);

        if print {
            print_chain(s, &chain);
        }
    }

    let mut histogram = HashMap::new();
    for c in chain {
        (*histogram.entry(c).or_insert(0)) += 1;
    }

    let mut maxcount = 0;
    let mut maxchar = 'a';
    let mut mincount = i32::MAX;
    let mut minchar = 'a';

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
        minchar,
        mincount,
        maxchar,
        maxcount,
        maxcount - mincount,
    );

    Ok(())
}

fn step(input: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut res = Vec::new();

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

fn print_chain(s: usize, chain: &Vec<char>) {
    print!("After step {}: ", s + 1);
    for c in chain {
        print!("{}", c);
    }
    println!();
}
