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

    let mut depth = 0;
    let mut position = 0;
    for l in reader.lines() {
        let line = l?;
        if line.starts_with("forward ") {
            position += rest(&line, "forward ")?;
        } else if line.starts_with("down ") {
            depth += rest(&line, "down ")?;
        } else if line.starts_with("up ") {
            depth -= rest(&line, "up ")?;
        } else {
            panic!("Unexpected directive");
        }
    }

    println!("The position is '{}', depth is '{}', product is '{}'.", position, depth, position*depth);
    Ok(())
}

fn rest(input: &str, start: &str) -> Result<i32, std::num::ParseIntError> {
    let rest = &input[start.len()..];
    Ok(rest.parse()?)
}