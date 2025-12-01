use std::io::BufRead;

#[derive(Debug)]
#[allow(dead_code)]
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
    let initial: Vec<_> = line.split(",").filter_map(|s| s.parse::<usize>().ok()).collect();

    let mut fish : Vec<i64> = vec!(0, 0, 0, 0, 0, 0, 0, 0, 0);

    for f in initial {
        fish[f] += 1;
    }

    println!("Initial state: {:?}", fish);
    for d in 0..256 {
        let mut new = Vec::new();
        for i in 1..9 {
            new.push(fish[i]);
        }

        new[6] += fish[0];
        new.push(fish[0]);

        fish = new;

        let count = fish.iter().fold(0, |f, a| f+a);
        println!("After {} days: {}", d+1, count);
    }

    //println!("There are now {} lanternfish", );
    Ok(())
}