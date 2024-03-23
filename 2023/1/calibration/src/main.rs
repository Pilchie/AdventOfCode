use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let (first, last) = parse_line_part_two(line);

        calibration_values.push((10 * first + last).into());
    }

    let result = calibration_values.into_iter().reduce(|acc, e| acc + e);

    println!("The value is {}", result.unwrap());
}

fn _parse_line_part_one(line: &str) -> (u8, u8) {
    let mut first: u8 = 255;
    let mut last: u8 = 0;
    for ch in line.bytes() {
        if ch >= b'0' && ch <= b'9' {
            if first == 255 {
                first = ch - b'0';
            }
            last = ch - b'0';
        }
    }
    (first, last)
}

fn parse_line_part_two(line: &str) -> (u8, u8) {
    let mut first: u8 = 255;
    let mut last: u8 = 0;

    let chars: Vec<u8> = line.bytes().collect();
    for i in 0..chars.len() {
        let ch = chars[i];
        if ch >= b'0' && ch <= b'9' {
            if first == 255 {
                first = ch - b'0';
            }
            last = ch - b'0';
        } else if chars[i..].starts_with(&[b'z', b'e', b'r', b'o']) {
            if first == 255 {
                first = 0;
            }
            last = 0;
        } else if chars[i..].starts_with(&[b'o', b'n', b'e']) {
            if first == 255 {
                first = 1;
            }
            last = 1;
        } else if chars[i..].starts_with(&[b't', b'w', b'o']) {
            if first == 255 {
                first = 2;
            }
            last = 2;
        } else if chars[i..].starts_with(&[b't', b'h', b'r', b'e', b'e']) {
            if first == 255 {
                first = 3;
            }
            last = 3;
        } else if chars[i..].starts_with(&[b'f', b'o', b'u', b'r']) {
            if first == 255 {
                first = 4;
            }
            last = 4;
        } else if chars[i..].starts_with(&[b'f', b'i', b'v', b'e']) {
            if first == 255 {
                first = 5;
            }
            last = 5;
        } else if chars[i..].starts_with(&[b's', b'i', b'x']) {
            if first == 255 {
                first = 6;
            }
            last = 6;
        } else if chars[i..].starts_with(&[b's', b'e', b'v', b'e', b'n']) {
            if first == 255 {
                first = 7;
            }
            last = 7;
        } else if chars[i..].starts_with(&[b'e', b'i', b'g', b'h', b't']) {
            if first == 255 {
                first = 8;
            }
            last = 8;
        } else if chars[i..].starts_with(&[b'n', b'i', b'n', b'e']) {
            if first == 255 {
                first = 9;
            }
            last = 9;
        }
    }

    (first, last)
}
