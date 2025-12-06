use std::{env, fs, ops::RangeInclusive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
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

    let mut ingredients = Vec::new();
    for line in contents.lines().skip(ranges.len() + 1) {
        let ingredient = line.parse::<u64>().unwrap();
        ingredients.push(ingredient);
    }

    part_one(&ranges, ingredients);
    part_two(&ranges);
}

fn part_one(ranges: &[RangeInclusive<u64>], ingredients: Vec<u64>) {
    let mut fresh = 0;

    for ingredient in ingredients {
        for range in ranges {
            if range.contains(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }
    println!("Part 1: {} of the available ingredient IDs are fresh", fresh);
}

fn part_two(ranges: &[RangeInclusive<u64>]) {
    let final_ranges = merge_ranges(ranges);
    let count: u64 = final_ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("Part 2: {} ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges", count);
}

fn merge_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for range in ranges {
        if let Some(last_range) = merged_ranges.last_mut() {
            if *range.start() <= *last_range.end() + 1 {
                let new_end = (*last_range.end()).max(*range.end());
                *last_range = RangeInclusive::new(*last_range.start(), new_end);
                continue;
            }
        }
        merged_ranges.push(range.clone());
    }

    merged_ranges
}
