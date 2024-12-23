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
    Ok(())
}

fn mix(input: i64, other: i64) -> i64 {
    input ^ other
}

fn prune(input: i64) -> i64 {
    input % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut secret = secret;
    let mult = secret *64;
    secret = prune(mix(secret, mult));
    let div = secret / 32;
    secret = prune(mix(secret, div));
    let mult2 = secret * 2048;
    secret = prune(mix(secret, mult2));
    secret
}
