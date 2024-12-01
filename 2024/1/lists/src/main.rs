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

    let mut part1 = 0;
    for i in 0..std::cmp::min(left.len(), right.len()) {
        part1 += (left[i] - right[i]).abs();
    }
    println!("{}", part1);

    let mut part2 = 0;
    for i in left {
        // TODO: Make use of the fact that right is sorted
        part2 += i * right.iter().filter(|&x| *x == i).count() as i32;
    }
    println!("{}", part2);
}