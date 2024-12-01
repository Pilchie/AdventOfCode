use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split("   ");
        let left_str = parts.next().unwrap();
        let right_str = parts.next().unwrap();

        let left_part = left_str.parse::<i32>().unwrap();
        let right_part = right_str.parse::<i32>().unwrap();
        left.push(left_part);
        right.push(right_part);
    }

    left.sort();
    right.sort();

    let mut accum = 0;
    for i in 0..std::cmp::min(left.len(), right.len()) {
        accum += (left[i] - right[i]).abs();
    }

    println!("{}", accum);
}