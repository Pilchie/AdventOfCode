use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
}

fn part_one(contents: &str) {
    let mut sum = 0;
    let mut rows = Vec::new();
    for line in contents.lines() {
        let row: Vec<_> = line.split_ascii_whitespace().collect();
        rows.push(row);
    }

    for i in 0..rows[0].len() {
        let op = rows[rows.len() - 1][i];
        let mut val = match op {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unknown operation"),
        };

        for j in 0..rows.len() - 1 {
            let num = rows[j][i].parse::<u64>().unwrap();
            match op {
                "+" => val += num,
                "*" => val *= num,
                _ => panic!("Unknown operation"),
            }
        }
        sum += val;
    }

    println!("Part 1: The grand total found by adding together all of the answers to the individual problems is {}", sum);
}
