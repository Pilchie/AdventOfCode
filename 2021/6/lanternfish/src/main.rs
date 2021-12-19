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
    let mut fish: Vec<_> = line.split(",").filter_map(|s| s.parse::<i32>().ok()).collect();

    println!("Initial state: {:?}", fish);
    for _ in 0..80 {
        let mut new = Vec::new();
        for f in &mut fish {
            if *f == 0 {
                new.push(8);
                *f = 6;
            } else {
                *f = *f - 1;
            }
        }

        fish.append(&mut new);
        //println!("After {} days: {:?}", d+1, fish);
    }

    println!("There are now {} lanternfish", fish.len());
    Ok(())
}