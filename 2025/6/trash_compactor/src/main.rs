use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
}

fn part_one(contents: &str) {
    let mut sum = 0;
    println!("Part 1: The grand total found by adding together all of the answers to the individual problems is {}", sum);
}
