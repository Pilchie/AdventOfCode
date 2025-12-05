use std::{env, fs, ops::RangeInclusive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
    part_two(&contents);
}

fn part_one(contents: &str) {
    let mut fresh = 0;

    let mut ranges = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            break;
        }
        let (l, r) = line.split_once('-').unwrap();
        let l = l.parse::<u64>().unwrap();
        let r = r.parse::<u64>().unwrap();
        ranges.push(RangeInclusive::new(l, r));
    }
    ranges.sort_by_key(|r| *r.start());

    for line in contents.lines().skip(ranges.len() + 1) {
        let ingredient = line.parse::<u64>().unwrap();
        for range in &ranges {
            if range.contains(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }
    println!("Part 1: {} of the available ingredient IDs are fresh", fresh);
}

fn part_two(contents: &str) {
    
}