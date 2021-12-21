use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

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

    let mut pairs = HashMap::new();
    for i in 0..template.len() - 1 {
        (*pairs.entry((template[i], template[i+1])).or_insert(0)) += 1;
    }

    for _ in 0..40 {
        pairs = step(&pairs, &rules);
    }


    let mut histogram = HashMap::new();
    for ((c1, c2), count) in pairs {
        (*histogram.entry(c1).or_insert(0)) += count;
        (*histogram.entry(c2).or_insert(0)) += count;
    }
    (*histogram.entry(template[template.len() -1]).or_insert(0)) += 1;


    let mut maxcount = 0;
    let mut maxchar = 'a';
    let mut mincount = u64::MAX;
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
        mincount/2,
        maxchar,
        maxcount / 2,
        (maxcount - mincount) / 2,
    );

    Ok(())
}

fn step(input: &HashMap<(char, char), u64>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u64> {
    let mut res = HashMap::new();

    for (k,v) in input {
        let x = rules.get(k).unwrap();
        (*res.entry((k.0, *x)).or_insert(0)) += v;
        (*res.entry((*x, k.1)).or_insert(0)) += v;
    }

    res
}
