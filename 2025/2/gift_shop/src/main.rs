use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
}

fn part_one(contents: &str) {
    let range_strs = contents.split(',');
    let ranges: Vec<_> = range_strs.map(|s| {
        let bounds = s.split_once('-').unwrap();
        let start: i64 = bounds.0.parse().unwrap();
        let end: i64 = bounds.1.parse().unwrap();
        std::ops::RangeInclusive::new(start, end)
    }).collect();
    let mut sum = 0;
    for range in ranges {
        for id in range {
            let s = id.to_string();
            if s.len() % 2 == 0 {
                let first_half = &s[..s.len() / 2];
                let second_half = &s[s.len() / 2..];
                if first_half == second_half {
                    //println!("Found invalid id: {}", id);
                    sum += id;
                }
            }
        }
    }
    println!("Part 1: The sum of all of the invalid ids is: {}", sum);
}
