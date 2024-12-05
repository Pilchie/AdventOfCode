use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            count += count_word_at(&lines, "XMAS", x, y);
            count += count_word_at(&lines, "SAMX", x, y);
        }
    }

    println!("XMAS appears {} times", count);

    count = 0;
    for y in 1..lines.len() - 1 {
        for x in 1..lines[y].len() - 1 {
            if matches_xmas(&lines, x, y) {
                count += 1;
            }
        }
    }

    println!("There are {} X-MASs", count);
}

fn count_word_at(lines: &[&str], word: &str, x: usize, y: usize) -> u32 {
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
        }
    }

    count
}

fn matches_xmas(lines: &[&str], x: usize, y: usize) -> bool {
    if lines[y].bytes().nth(x) == Some(b'A') {
        if lines[y - 1].bytes().nth(x - 1) == Some(b'M')
            && lines[y + 1].bytes().nth(x + 1) == Some(b'S')
            || lines[y - 1].bytes().nth(x - 1) == Some(b'S')
                && lines[y + 1].bytes().nth(x + 1) == Some(b'M')
        {
            if lines[y - 1].bytes().nth(x + 1) == Some(b'M')
                && lines[y + 1].bytes().nth(x - 1) == Some(b'S')
                || lines[y - 1].bytes().nth(x + 1) == Some(b'S')
                    && lines[y + 1].bytes().nth(x - 1) == Some(b'M')
            {
                return true;
            }
        }
    }

    false
}
