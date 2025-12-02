use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let range_strs = contents.split(',');
    let ranges: Vec<_> = range_strs
        .map(|s| {
            let bounds = s.split_once('-').unwrap();
            let start: i64 = bounds.0.parse().unwrap();
            let end: i64 = bounds.1.parse().unwrap();
            std::ops::RangeInclusive::new(start, end)
        })
        .collect();
    part_one(&ranges);
    part_two(&ranges);
}
fn part_one(ranges: &[std::ops::RangeInclusive<i64>]) {
    let mut sum = 0;
    for range in ranges {
        for id in *range.start()..=*range.end() {
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

fn part_two(ranges: &[std::ops::RangeInclusive<i64>]) {
    let mut sum = 0;
    for range in ranges {
        for id in *range.start()..=*range.end() {
            if is_repeated_sequence(id) {
                sum += id;
            }
        }
    }
    println!("Part 2: The sum of all of the invalid ids is: {}", sum);
}

fn is_repeated_sequence(id: i64) -> bool {
    let s = id.to_string();
    for pattern_length in 1..=s.len() / 2 {
        let pattern = &s[..pattern_length];
        let mut repeated = true;
        for i in (0..s.len()).step_by(pattern_length) {
            let end = std::cmp::min(i + pattern_length, s.len());
            if &s[i..end] != pattern {
                repeated = false;
                break;
            }
        }
        if repeated {
            println!("Found repeated sequence: {}", id);
            return true;
        }
    }
    false
}
