use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut stones: Vec<String> = contents.split(' ').map(|s| s.to_string()).collect();
    println!("Initial arrangement:");
    println!("{:?}", stones);
    for b in 0..25 {
        stones = blink(&stones);
        println!("After {} blink(s): {}", b + 1, stones.len());
        //println!("{:?}", stones);
    }
}

fn blink(stones: &[String]) -> Vec<String> {
    let mut res = Vec::new();
    res.reserve(stones.len());

    for s in stones {
        if let Ok(val) = s.parse::<u64>() {
            if val == 0 {
                res.push("1".to_string());
            } else if s.len() % 2 == 0 {
                add_string_val(&mut res, &s[0..s.len() / 2]);
                add_string_val(&mut res, &s[s.len() / 2..]);
            } else {
                res.push(format!("{}", val * 2024));
            }
        }
    }
    res
}

fn add_string_val(stones: &mut Vec<String>, s: &str) {
    let val = s.parse::<u64>().unwrap();
    stones.push(format!("{}", val));
}
