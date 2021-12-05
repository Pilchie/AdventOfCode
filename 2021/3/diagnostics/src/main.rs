use std::io::BufRead;

#[derive(Debug)]
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

    let mut bit_counts = vec!(0,0,0,0,0,0,0,0,0,0,0,0);
    let mut count = 0;
    for l in reader.lines() {
        let line = l?;
        count += 1;
        for (i, c) in line.char_indices() {
            if c == '1' {
                bit_counts[i] += 1;
            }
        }
    }

    println!("Count is '{}'", count);
    println!("{:?}", bit_counts);

    let mut epsilon = 0;
    let mut gamma = 0;
    for i in 0..bit_counts.len() {
        if bit_counts[i] > count / 2 {
            gamma += 1 << (bit_counts.len() - 1 - i);
        } else {
            epsilon += 1 << (bit_counts.len() - 1 - i);
        }
    }

    println!("epsilon is '{:b}', gamma is '{:b}', product is '{}'", epsilon, gamma, epsilon * gamma);

    Ok(())
}