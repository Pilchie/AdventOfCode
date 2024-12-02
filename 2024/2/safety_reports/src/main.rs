use std::env;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let reader = BufReader::new(std::fs::File::open(&args[1])?);

    let mut safe = 0;
    for line in reader.lines() {
        if is_safe(&line?) {
            safe += 1;
        }
    }

    println!("{} safe reports", safe);

    Ok(())
}

fn is_safe(report: &str) -> bool {
    let mut ascending: Option<bool> = None;
    let mut prev: Option<u32> = None;
    for val_str in report.split_whitespace() {
        let val = val_str.parse::<u32>().unwrap();
        if let Some(p) = prev {
            match ascending {
                Some(true) => {
                    if val <= p || val - p > 3 {
                        return false;
                    }
                }
                Some(false) => {
                    if val >= p || p - val > 3 {
                        return false;
                    }
                }
                None => {
                    ascending = Some(val > p);
                }
            }
        }
        prev = Some(val);
    }
    true
}
