use std::{env, fs, num::ParseIntError};

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
    let sequences = sequences();
    println!("Found {} sequences to try", sequences.len());
    for sequence in sequences {
        println!("Trying sequence {:?}", sequence);
        let mut bananas = 0;
        for line in contents.lines() {
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
                if [diff1, diff2, diff3, diff4] == sequence {
                    bananas += secret % 10;
                    //println!("For sequence {:?} - secret {}, found a match: Got {}", sequence, first, secret % 10);
                    break; 
                }
                diff1 = diff2;
                diff2 = diff3;
                diff3 = diff4;
                prev = secret;
            }
        }
        if bananas > max_bananas {
            max_bananas = bananas;
        }
    }
    println!("The most bananas you can get is: {}", max_bananas);
    Ok(())
}

fn diff(new: i64, old: i64) -> i64 {
    new % 10 - old % 10
}

fn sequences() -> Vec<[i64; 4]> {
    let mut res = Vec::new();
    let mut first: i64 = -9;
    println!("Starting to generate sequences");
    while first <= 9 {
        let mut second = -9;
        while second <= 9 {
            if (first + second).abs() <= 9 {
                let mut third = -9;
                while third <= 9 {
                    if (second + third).abs() <= 9 {
                        let mut fourth = -9;
                        while fourth <= 9 {
                            if (third + fourth).abs() <= 9 {
                                res.push([first, second, third, fourth]);
                            }
                            fourth += 1;
                        }
                    }
                    third += 1;

                }
            }
            second += 1;
        }
        first += 1;
    }

    res
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
