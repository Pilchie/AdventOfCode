use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1])?);

    let input : Vec<_> = reader.lines().map(|line| line.unwrap().parse::<usize>().unwrap()).collect();
    if let Some(weakness) = encryption_weakness(&input, 25) {
        println!("Found encryption weakness: {}", weakness);
    } else {
        println!("Didn't find an encryption weakness.");
    }

    Ok(())
}

pub fn is_last_valid(num: usize, input: &[usize]) -> bool {
    for x in 0..input.len() - 1 {
        for y in x + 1..input.len() {
            //if num == 863 {
            //    println!("Checking {} + {} = {},", input[x], input[y], input[x] + input[y]);
            //}
            if input[x] + input[y] == num && input[x] != input[y] {
                return true;
            }
        }
    }

    false
}

pub fn first_invalid(input: &[usize], preamble_length: usize) -> Option<(usize, usize)> {
    // Consider - storing a set of previous sums, along with the indices of their addends would allow
    // fewer iterations if the input was super long, or the preamble length was longer.
    for x in preamble_length..input.len() {
        //println!("looking at index {} from {} to {}", x, x-preamble_length, x);
        if !is_last_valid(input[x], &input[x-preamble_length..x]) {
            return Some((x,input[x]));
        }
    }

    None
}

pub fn encryption_weakness(input: &[usize], preamble_size: usize) -> Option<usize> {
    let (index, first_invalid) = first_invalid(input, preamble_size)?;

    // Let's just brute force this - how many subsets can there be?
    for x in 0..index-1 {
        for y in x+1..index {
            let val = input[x..y].iter().fold(0, |acc, v| acc+v);
            if val == first_invalid {
                let min = input[x..y].iter().min()?;
                let max = input[x..y].iter().max()?;
                return Some(min + max);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_combos1() {
        // Numbers from 1-25 in random order
        let input = [20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24, 25];
        assert!(is_last_valid(26, &input));
        assert!(is_last_valid(49, &input));
        assert!(!is_last_valid(100, &input));
        assert!(!is_last_valid(50, &input));
    }

    #[test]
    fn test_valid_combos2() {
        // Numbers from 1-25 in random order
        let input = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24, 25, 45];
        assert!(is_last_valid(26, &input));
        assert!(!is_last_valid(65, &input));
        assert!(is_last_valid(64, &input));
        assert!(is_last_valid(66, &input));
    }

    #[test]
    fn test_sequence() {
        let input = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,];
        assert_eq!(Some((14, 127)), first_invalid(&input, 5));
    }

    #[test]
    fn from_input() {
        let input = [175, 182, 209, 191, 192, 195, 206, 297, 215, 218, 229, 265, 257, 262, 287, 368, 293, 302, 321, 331, 371, 338, 392, 367, 439, 542,];
        assert!(is_last_valid(863, &input));
    }

    #[test]
    fn weakness() {
        let input = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,];
        assert_eq!(Some(62), encryption_weakness(&input, 5));
    }
}