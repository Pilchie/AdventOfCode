use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let (locks, keys) = parse(&contents);
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if key.fits(lock) {
                count += 1;
            }
        }
    }
    println!("There are {} pairs", count);
}

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "#####" {
            locks.push(Lock::parse(&lines, i));
        } else if lines[i] == "....." {
            keys.push(Key::parse(&lines, i));
        } else {
            unreachable!();
        }
        i += 8;
    }
    (locks, keys)
}

struct Lock {
    heights: [u8; 5],
}

impl Lock {
    fn parse(input: &[&str], start: usize) -> Self {
        let mut res = Self {
            heights: [0, 0, 0, 0, 0],
        };

        for r in 1..6 {
            for (col, ch) in input[start + r].bytes().enumerate() {
                if ch == b'#' {
                    res.heights[col] += 1;
                }
            }
        }
        res
    }
}

struct Key {
    heights: [u8; 5],
}

impl Key {
    fn parse(input: &[&str], start: usize) -> Self {
        let mut res = Self {
            heights: [0, 0, 0, 0, 0],
        };

        for r in 1..6 {
            for (col, ch) in input[start + r].bytes().enumerate() {
                if ch == b'#' {
                    res.heights[col] += 1;
                }
            }
        }
        res
    }
    
    fn fits(&self, lock: &Lock) -> bool {
        for i in 0..self.heights.len() {
            if self.heights[i] + lock.heights[i] > 5 {
                return false;
            }
        }
        true
    }
}
