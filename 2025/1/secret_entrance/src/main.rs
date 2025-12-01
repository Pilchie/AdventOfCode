use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

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

    println!("The correct password is: {}", sum);
}
