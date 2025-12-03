use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
}

fn part_one(contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        let mut max = line[0..1].parse::<u8>().unwrap();
        let mut idx_max = 0;
        for (idx, ch) in line[1..line.len()-1].bytes().enumerate() {
            let cur = ch - b'0';
            if cur > max {
                max = cur;
                idx_max = idx + 1; // +1 because we started from line[1..]
            }
        }
        let second = line[idx_max+1..line.len()].bytes().map(|b| b - b'0').max().unwrap();
        let val = max as u32 * 10 + second as u32;
        //println!("Joltage for {} is '{}{}'. idx_max is {}", line, max, second, idx_max);
        sum += val;
    }
    println!("Part 1: The total output joltage is: {}", sum);
}