use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IO(ioe)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(pie: std::num::ParseIntError) -> Self {
        Error::Parse(pie)
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    let reader = std::io::BufReader::new(std::fs::File::open(&args[1])?);

    let mut bit_counts = HashMap::new();
    let mut count = 0;
    let mut numbers = Vec::new();
    for l in reader.lines() {
        let line = l?;
        count += 1;
        for (i, c) in line.char_indices() {
            let b = bit_counts.entry(i).or_insert(0);
            if c == '1' {
                *b += 1;
            }
        }

        let mut cur = 0;
        for (i, c) in line.char_indices() {
            if c == '1' {
                cur += 1 << bit_counts.keys().len() - 1 - i;
            }
        }
        numbers.push(cur);
    }

    let mut epsilon = 0;
    let mut gamma = 0;
    let mut o2_generator = numbers.clone();
    let mut co2_scrubber = numbers.clone();
    for i in 0..bit_counts.len() {
        let mask = bit_counts.len() - 1 - i;
        if bit_counts[&i] > count / 2 {
            gamma += 1 << mask;
        } else {
            epsilon += 1 << mask;
        }

        o2_generator = filter(&mut o2_generator, i, bit_counts.len(), false);
        co2_scrubber = filter(&mut co2_scrubber, i, bit_counts.len(), true);
    }

    println!("epsilon is '{:b}', gamma is '{:b}', product is '{}'", epsilon, gamma, epsilon * gamma);

    let o2_rate = o2_generator.first().unwrap();
    let co2_rate = co2_scrubber.first().unwrap();
    println!("o2_rate is '{:b}', co2_rate is '{:b}', product is '{}'", o2_rate, co2_rate, o2_rate * co2_rate);

    Ok(())
}

fn filter(numbers: &mut Vec<i32>, bit: usize, word_length: usize, invert: bool) -> Vec<i32> {
    if numbers.len() == 1 {
        return numbers.clone();
    }

    let mask = word_length - 1 - bit;
    let bits = numbers.iter().filter(|x| *x & 1 << mask != 0).count();

    let value_to_match = match (bits * 2).cmp(&numbers.len()) {
        Ordering::Less => match invert {
            false => 0,
            true => 1,
        },
        Ordering::Equal => match invert {
            false => 1,
            true => 0,
        },
        Ordering::Greater => match invert {
            false => 1,
            true => 0,
        },
    };

    numbers.iter().filter(|x| {
        *x & 1 << mask == value_to_match << mask
    }).map(|x| *x).collect()
}