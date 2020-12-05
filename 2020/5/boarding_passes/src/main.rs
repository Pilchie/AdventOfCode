use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    // if let Ok(max) = max_id(reader) {
    //     println!("The largest id is {}", max);
    // }

    let mut all_passes = HashSet::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            let bp = BoardingPass::new(&l)?;
            all_passes.insert(bp.seat_id());
        }
    }

    for row in 1..127 {
        for col in 0..7 {
            let id = 8 * row + col;
            if !all_passes.contains(&id)
                && all_passes.contains(&(id - 1))
                && all_passes.contains(&(id + 1))
            {
                println!("Your seat id is {}", id);
            }
        }
    }

    Ok(())
}

fn max_id(reader: io::BufReader<std::fs::File>) -> Result<usize, Error> {
    let mut max = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let bp = BoardingPass::new(&l)?;
            let id = bp.seat_id();
            if id > max {
                max = id
            }
        }
    }

    Ok(max)
}

pub struct BoardingPass {
    definition: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidBoardingPass,
    IOError(std::io::Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}

impl BoardingPass {
    pub fn new(definition: &str) -> Result<BoardingPass, Error> {
        if definition.len() == 10
            && chars_in(&definition[0..7], &['F', 'B'])
            && chars_in(&definition[8..10], &['L', 'R'])
        {
            Ok(BoardingPass {
                definition: definition.into(),
            })
        } else {
            Err(Error::InvalidBoardingPass)
        }
    }

    pub fn row(&self) -> usize {
        bsp(&self.definition[0..7], 'B', 'F')
    }

    pub fn col(&self) -> usize {
        bsp(&self.definition[7..10], 'R', 'L')
    }

    pub fn seat_id(&self) -> usize {
        8 * self.row() + self.col()
    }
}

fn chars_in(val: &str, allowed: &[char]) -> bool {
    for c in val.chars() {
        if !allowed.contains(&c) {
            return false;
        }
    }

    true
}

fn bsp(val: &str, up: char, down: char) -> usize {
    let mut min = 0;
    let mut max = (2 << val.len() - 1) - 1;
    let mut mid = max / 2;

    //println!("Initial: min: {}, mid: {}, max: {}", min, mid, max);
    for c in val.chars() {
        if c == up {
            min = mid + 1;
        } else if c == down {
            max = mid;
        } else {
            panic!();
        }

        mid = (min + max) / 2;

        //println!("After {} - min: {}, mid: {}, max: {}", c, min, mid, max);
    }

    mid
}

#[cfg(test)]
mod test {
    use super::{BoardingPass, Error};

    #[test]
    fn example1() -> Result<(), Error> {
        let boarding_pass = BoardingPass::new("FBFBBFFRLR")?;
        assert_eq!(44, boarding_pass.row());
        assert_eq!(5, boarding_pass.col());
        assert_eq!(357, boarding_pass.seat_id());
        Ok(())
    }

    #[test]
    fn example2() -> Result<(), Error> {
        let boarding_pass = BoardingPass::new("BFFFBBFRRR")?;
        assert_eq!(70, boarding_pass.row());
        assert_eq!(7, boarding_pass.col());
        assert_eq!(567, boarding_pass.seat_id());
        Ok(())
    }

    #[test]
    fn example3() -> Result<(), Error> {
        let boarding_pass = BoardingPass::new("FFFBBBFRRR")?;
        assert_eq!(14, boarding_pass.row());
        assert_eq!(7, boarding_pass.col());
        assert_eq!(119, boarding_pass.seat_id());
        Ok(())
    }

    #[test]
    fn example4() -> Result<(), Error> {
        let boarding_pass = BoardingPass::new("BBFFBBFRLL")?;
        assert_eq!(102, boarding_pass.row());
        assert_eq!(4, boarding_pass.col());
        assert_eq!(820, boarding_pass.seat_id());
        Ok(())
    }
}
