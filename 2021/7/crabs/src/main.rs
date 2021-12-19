use std::io::BufRead;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
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

    let line = reader.lines().last().ok_or(Error::Custom("couldn't parse!".to_string()))??;
    let crabs: Vec<_> = line.split(",").filter_map(|s| s.parse::<i32>().ok()).collect();

    let min = crabs.iter().fold(i32::MAX, |m, c| if c < &m { return *c; } else { return m; });
    let max = crabs.iter().fold(i32::MIN, |m, c| if c > &m { return *c; } else { return m; });

    let mut min_fuel = i32::MAX;
    for i in min..max+1 {
        let mut sum = 0;
        for c in &crabs {
            let dist = (c - i).abs();
            let cost = dist * (dist + 1) / 2;
            sum += cost;
        }

        if sum < min_fuel {
            min_fuel = sum;
        }
    }

    println!("Minimum fuel is {}", min_fuel);

    Ok(())
}