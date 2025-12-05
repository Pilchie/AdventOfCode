use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
    part_two(&contents);
}

fn part_one(contents: &str) {
    println!("Part 1:");
}

fn part_two(contents: &str) {
    
}