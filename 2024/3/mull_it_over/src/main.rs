use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let chars = contents.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut result = 0;
    while i < chars.len() - 8 {
        if let Some((val, len)) = parse_mul(&chars, i) {
            result += val;
            i += len;
        } else {
            i += 1;
        }
    }

    println!("Result: {}", result);
}

fn parse_mul(chars: &[char], i: usize) -> Option<(u32, usize)> {
    if chars[i..i + "mul(".len()] == ['m', 'u', 'l', '('] {
        if let Some((left, llen)) = parse_num(&chars, i + "mul(".len()) {
            if chars[i + "mul(".len() + llen] == ',' {
                if let Some((right, rlen)) = parse_num(&chars, i + llen + "mul(,".len()) {
                    if chars[i + "mul(,".len() + llen + rlen] == ')' {
                        return Some((left * right, llen + rlen + "mul(,)".len()));
                    }
                }
            }
        }
    }

    None
}

fn parse_num(chars: &[char], index: usize) -> Option<(u32, usize)> {
    let mut len = 0;
    let mut val = None;
    while index + len < chars.len() {
        if !chars[index + len].is_ascii_digit() || len == 3 {
            let s = String::from_iter(chars[index..index + len].iter());
            val = Some(s.parse::<u32>().unwrap());
            break;
        }
        len += 1;
    }

    if len == 0 {
        return None;
    }

    Some((val.unwrap(), len))
}
