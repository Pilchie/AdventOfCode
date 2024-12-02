use std::env;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let reader = BufReader::new(std::fs::File::open(&args[1])?);

    let mut safe = 0;
    for line in reader.lines() {
        if is_safe(&line?, true) {
            safe += 1;
        }
    }

    println!("{} safe reports", safe);

    Ok(())
}

fn is_safe(report: &str, allow_dampening: bool) -> bool {
    let values: Vec<_> = report
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    if allow_dampening {
        for i in 0..values.len() {
            let mut values = values.clone();
            values.remove(i);

            if is_safe_values(&values) {
                return true;
            }
        }
    }

    is_safe_values(&values)
}

fn is_safe_values(values: &[u32]) -> bool {
    let mut ascending: Option<bool> = None;
    let mut prev: Option<u32> = None;

    for val in values {
        if let Some(p) = prev {
            if ascending.is_none() {
                ascending = Some(*val > p);
            }
            match ascending {
                Some(true) => {
                    if *val <= p || val - p > 3 {
                        return false;
                    }
                }
                Some(false) => {
                    if *val >= p || p - val > 3 {
                        return false;
                    }
                }
                None => unreachable!(),
            }
        }
        prev = Some(*val);
    }

    true
}
