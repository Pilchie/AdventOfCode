use std::{collections::{HashMap, HashSet}, env, fs, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        let mut secret = line.parse::<i64>()?;
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        sum += secret;
    }
    println!("The sum of the 2000th secret of each buyer is {}", sum);

    // Part 2
    let mut max_bananas = 0;
    let mut all_sequences = HashSet::new();
    let mut sequences_by_monkey = Vec::new();

    for line in contents.lines() {
        let mut counts_by_sequence = HashMap::new();
        let first = line.parse::<i64>()?;
        let second = next_secret(first);
        let third = next_secret(second);
        let mut prev = next_secret(third);
        let mut diff1 = diff(second, first);
        let mut diff2 = diff(third, second);
        let mut diff3 = diff(prev, third);
        for _ in 4..2000 {
            let secret = next_secret(prev);
            let diff4 = diff(secret, prev);
            let sequence = [diff1, diff2, diff3, diff4];
            all_sequences.insert(sequence);
            if !counts_by_sequence.contains_key(&sequence) {
                counts_by_sequence.insert(sequence, secret % 10);
            }
            diff1 = diff2;
            diff2 = diff3;
            diff3 = diff4;
            prev = secret;
        }
        sequences_by_monkey.push(counts_by_sequence);
    }

    for sequence in all_sequences {
        let mut bananas_sequence = 0;
        for monkey in &sequences_by_monkey {
            if let Some(bananas_monkey) = monkey.get(&sequence) {
                bananas_sequence += bananas_monkey;
            }
        }
        if bananas_sequence > max_bananas {
            max_bananas = bananas_sequence;
        }
    }
    println!("The most bananas you can get is: {}", max_bananas);
    Ok(())
}

fn diff(new: i64, old: i64) -> i64 {
    new % 10 - old % 10
}

fn mix(input: i64, other: i64) -> i64 {
    input ^ other
}

fn prune(input: i64) -> i64 {
    input % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut secret = secret;
    let mult = secret * 64;
    secret = prune(mix(secret, mult));
    let div = secret / 32;
    secret = prune(mix(secret, div));
    let mult2 = secret * 2048;
    secret = prune(mix(secret, mult2));
    secret
}
