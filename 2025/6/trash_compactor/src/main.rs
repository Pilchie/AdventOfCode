use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
    part_two(&contents);
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

fn part_two(contents: &str) {
    let mut sum = 0;
    let lines: Vec<&str> = contents.lines().collect();

    let mut cols = Vec::new();
    for i in (0..lines[0].len()).rev() {
        let col: Vec<_> = lines.iter().map(|line| &line[i..i+1]).collect();
        cols.push(col);
    }

    let mut curr = Vec::new();
    for c in 0..cols.len() {
        let col = &cols[c];
        if col.iter().all(|c| c.trim().is_empty())
        {
            let op = cols[c - 1][cols[c - 1].len() - 1];
            sum += compute_value(&curr, op);
            curr.clear();
        } else {
            let mut val = 0;
            for r in 0..col.len() - 1 {
                if col[r].trim().is_empty() {
                    continue;
                }
                let num = col[r].parse::<u64>().unwrap();
                val = val * 10 + num;
            }
            curr.push(val);
        }
    }
    sum += compute_value(&curr, cols[cols.len() - 1][cols[cols.len() - 1].len() - 1]);

    println!("Part 2: The grand total found by adding together all of the answers to the individual problems is {}", sum);
}

fn compute_value(curr: &Vec<u64>, op: &str) -> u64 {
    let val = match op {
        "+" => curr.iter().sum::<u64>(),
        "*" => curr.iter().product::<u64>(),
        _ => panic!("Unknown operation"),
    };
    val
}
