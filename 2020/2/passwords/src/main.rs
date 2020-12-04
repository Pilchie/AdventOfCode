use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use passwords::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]);

    let reader = io::BufReader::new(file.unwrap());
    let mut total = 0;
    let mut bad = 0;
    for line in reader.lines() {
        total += 1;
        let pwl = PasswordLine::parse(&line.unwrap());
        if !pwl.is_valid_part2() {
            bad += 1;
            println!("Found an invalid password line: {:#?}", pwl);
        }
    }
    println!("Found {} bad passwords out of {}, {} good", bad, total, total - bad);
}
