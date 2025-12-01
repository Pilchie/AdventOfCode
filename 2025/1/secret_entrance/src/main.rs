use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
    part_two(&contents);
}

fn part_one(contents: &str) {
    let mut pos = 50;
    let mut sum = 0;
    for line in contents.lines() {
        let num: i32 = line[1..].parse().unwrap();
        pos += match &line[0..1] {
            "L" => -1 * num,
            "R" => num,
            _ => panic!("Unknown direction"),
        };

        pos = pos % 100;
        if pos < 0 {
            pos += 100;
        }

        if pos == 0 {
            sum += 1;
        }
    }

    println!("Part 1: The actual password to open the door is: {}", sum);
}

fn part_two(contents: &str) {
    let mut pos = 50;
    let mut sum = 0;
    for line in contents.lines() {
        let num: i32 = line[1..].parse().unwrap();
        for _ in 0..num {
            pos += match &line[0..1] {
                "L" => -1,
                "R" => 1,
                _ => panic!("Unknown direction"),
            };

            if pos == 0 {
                sum += 1;
            } else if pos == 100 {
                pos = 0;
                sum += 1;
            } else if pos == -1 {
                pos = 99;
            }
        }
        //println!("{} - Current position: {}, current sum: {}", line, pos, sum);
    }

    println!("Part 2: Using password method 0x434C49434B, the password to open the door is: {}", sum);
}
