use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1])?);
    let input: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();
    println!("The number of combos is {}", combos(&input));

    Ok(())
}

pub fn ratings(adapters: &[usize]) -> usize {
    let mut sorted = adapters.to_vec();
    sorted.sort();

    let mut ones = 0;
    let mut threes = 1;
    let mut prev = 0;

    for x in sorted {
        match x - prev {
            1 => ones += 1,
            2 => {}
            3 => threes += 1,
            _ => panic!(),
        };
        prev = x;
    }

    ones * threes
}

pub fn combos(adapters: &[usize]) -> usize {
    let mut sorted = adapters.to_vec();
    sorted.push(0);
    sorted.push(sorted.iter().max().unwrap() + 3);
    sorted.sort();

    let mut next: Vec<usize> = Vec::new();
    let mut count = 0;
    next.push(0);

    println!("Dealing with a length of {}", sorted.len());

    let mut step = 1;
    while !next.is_empty() {
        let cur = next.clone();
        next.clear();
        let mut stepcount = 0;
        for index in cur {
            if index == sorted.len() - 1 {
                stepcount += 1;
            } else {
                for x in index + 1..min(index + 4, sorted.len()) {
                    //println!("Checking items at index {} and {}", x, index);
                    if sorted[x] - sorted[index] <= 3 {
                        next.push(x);
                    }
                }
            }
        }

        println!("Finished step {}, pushed {} to next, ended {}", step, next.len(), stepcount);
        step += 1;
        count += stepcount;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(35, ratings(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4,]));
    }

    #[test]
    fn example2() {
        assert_eq!(
            220,
            ratings(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ])
        );
    }

    #[test]
    fn example1_part2() {
        assert_eq!(8, combos(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4,]));
    }

    #[test]
    fn example2_part2() {
        assert_eq!(
            19208,
            combos(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47,
                24, 23, 49, 45, 19, 38, 39, 11, 1, 32,
                25, 35, 8, 17, 7, 9, 4, 2, 34, 10,
                3,
            ])
        );
    }
}
