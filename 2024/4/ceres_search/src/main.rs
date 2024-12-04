use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            count += count_at(&lines, "XMAS", x, y);
            count += count_at(&lines, "SAMX", x, y);
        }
    }

    println!("{}", count);
}

fn count_at(lines: &[&str], word: &str, x: usize, y: usize) -> u32 {
    let mut count = 0;

    // Check to the right
    if x + word.len() <= lines[y].len() {
        let mut found = true;
        for i in 0..word.len() {
            if lines[y].bytes().nth(x + i) != word.bytes().nth(i) {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
            //println!("Found at {},{} - {} right", y, x, word);
        }
    }

    // Check down
    if y + word.len() <= lines.len() {
        let mut found = true;
        for i in 0..word.len() {
            if lines[y + i].bytes().nth(x) != word.bytes().nth(i) {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
            //println!("Found at {},{} - {} down", y, x, word);
        }
    }

    // Check right diagonal
    if x + word.len() <= lines[y].len() && y + word.len() <= lines.len() {
        let mut found = true;
        for i in 0..word.len() {
            if lines[y + i].bytes().nth(x + i) != word.bytes().nth(i) {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
            //println!("Found at {},{} - {} diag right", y, x, word);
        }
    }

    // Check left diagonal
    if x >= word.len() - 1 && y + word.len() <= lines.len() {
        let mut found = true;
        for i in 0..word.len() {
            if lines[y + i].bytes().nth(x - i) != word.bytes().nth(i) {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
            //println!("Found at {},{} - {} diag left", y, x, word);
        }
    }

    count
}
