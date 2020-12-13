use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1])?);
    let script = Action::parse_script_from_bufread(&mut reader)?;

    let mut ship = Ship::new();
    for action in script {
        ship = ship.apply(&action);
    }

    println!("The ship is at a manhattan distance of {}", ship.manhattan_distance());

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
    TooShort,
    WrongDirection(char),
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

pub enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl Action {
    pub fn parse_script_from_string(script: &str) -> Result<Vec<Self>, Error> {
        let mut res = Vec::new();
        for line in script.lines() {
            res.push(Action::parse(line)?);
        }
        Ok(res)
    }

    pub fn parse_script_from_bufread<T: BufRead>(script: &mut T) -> Result<Vec<Self>, Error> {
        let mut res = Vec::new();
        for line in script.lines() {
            res.push(Action::parse(&line?)?);
        }
        Ok(res)
    }

    pub fn parse(input: &str) -> Result<Self, Error> {
        let val = input[1..].parse::<isize>()?;
        match input.chars().nth(0) {
            Some(c) => match c {
                'N' => Ok(Action::North(val)),
                'S' => Ok(Action::South(val)),
                'E' => Ok(Action::East(val)),
                'W' => Ok(Action::West(val)),
                'L' => Ok(Action::Left(val)),
                'R' => Ok(Action::Right(val)),
                'F' => Ok(Action::Forward(val)),
                _ => Err(Error::WrongDirection(c)),
            },
            None => Err(Error::TooShort),
        }
    }
}

pub struct Ship {
    x: isize,
    y: isize,
    heading: isize,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 0,
        }
    }
    pub fn apply(&self, action: &Action) -> Self {
        match action {
            Action::North(val) => Self {
                x: self.x,
                y: self.y - val,
                heading: self.heading,
            },
            Action::South(val) => Self {
                x: self.x,
                y: self.y + val,
                heading: self.heading,
            },
            Action::East(val) => Self {
                x: self.x + val,
                y: self.y,
                heading: self.heading,
            },
            Action::West(val) => Self {
                x: self.x - val,
                y: self.y,
                heading: self.heading,
            },
            Action::Left(val) => Self {
                x: self.x,
                y: self.y,
                heading: (self.heading + val) % 360,
            },
            Action::Right(val) => Self {
                x: self.x,
                y: self.y,
                heading: (self.heading - val) % 360,
            },
            Action::Forward(val) => {
                if self.heading == 0 {
                    Self {
                        x: self.x + val,
                        y: self.y,
                        heading: self.heading,
                    }
                } else if self.heading == 90 || self.heading == -270 {
                    Self {
                        x: self.x,
                        y: self.y - val,
                        heading: self.heading,
                    }
                } else if self.heading == 180 || self.heading == -180 {
                    Self {
                        x: self.x - val,
                        y: self.y,
                        heading: self.heading,
                    }
                } else if self.heading == 270 || self.heading == -90 {
                    Self {
                        x: self.x,
                        y: self.y + val,
                        heading: self.heading,
                    }
                } else {
                    panic!("Unhandled heading of {}", self.heading);
                }
            }
        }
    }

    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() -> Result<(), Error> {
        let script = Action::parse_script_from_string(
            "F10
N3
F7
R90
F11",
        )?;
        let mut ship = Ship::new();
        for action in script {
            ship = ship.apply(&action);
        }

        assert_eq!(25, ship.manhattan_distance());

        Ok(())
    }
}
