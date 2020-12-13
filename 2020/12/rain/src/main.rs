use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1])?);
    let script = Action::parse_script_from_bufread(&mut reader)?;

    let mut ship = Ship::new();
    for action in script {
        ship = ship.apply_part2(&action);
    }

    println!(
        "The ship is at a manhattan distance of {}",
        ship.manhattan_distance()
    );

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

#[derive(Clone, Copy)]
pub struct WayPoint {
    x: isize,
    y: isize,
}

impl WayPoint {
    pub fn new() -> Self {
        Self { x: 10, y: 1 }
    }
}

pub struct Ship {
    x: isize,
    y: isize,
    heading: isize,
    waypoint: WayPoint,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: 0,
            waypoint: WayPoint::new(),
        }
    }

    pub fn with_x(&self, dx: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y,
            heading: self.heading,
            waypoint: self.waypoint,
        }
    }

    pub fn with_y(&self, dy: isize) -> Self {
        Self {
            x: self.x,
            y: self.y + dy,
            heading: self.heading,
            waypoint: self.waypoint,
        }
    }

    pub fn with_heading(&self, dh: isize) -> Self {
        Self {
            x: self.x,
            y: self.y,
            heading: (self.heading + dh) % 360,
            waypoint: self.waypoint,
        }
    }

    pub fn with_waypoint_x(&self, dx: isize) -> Self {
        Self {
            x: self.x,
            y: self.y,
            heading: self.heading,
            waypoint: WayPoint {
                x: self.waypoint.x + dx,
                y: self.waypoint.y,
            },
        }
    }

    pub fn with_waypoint_y(&self, dy: isize) -> Self {
        Self {
            x: self.x,
            y: self.y,
            heading: self.heading,
            waypoint: WayPoint {
                x: self.waypoint.x,
                y: self.waypoint.y + dy,
            },
        }
    }

    pub fn with_waypoint_rotate(&self, dr: isize) -> Self {
        let rads = (dr as f64).to_radians();
        let cos_theta = rads.cos();
        let sin_theta = rads.sin();

        // Note - because we're storing the waypoint as relative to the ship,
        // we don't have to subtract the ship's components in the multiplication below.
        let wx = cos_theta * self.waypoint.x as f64
            - sin_theta * self.waypoint.y as f64;
        let wy = sin_theta * self.waypoint.x as f64
            + cos_theta * self.waypoint.y as f64;

        Self {
            x: self.x,
            y: self.y,
            heading: self.heading,
            waypoint: WayPoint {
                x: unsafe { wx.round().to_int_unchecked() },
                y: unsafe { wy.round().to_int_unchecked() },
            },
        }
    }

    pub fn apply(&self, action: &Action) -> Self {
        match action {
            Action::North(val) => self.with_y(*val),
            Action::South(val) => self.with_y(-val),
            Action::East(val) => self.with_x(*val),
            Action::West(val) => self.with_x(-val),
            Action::Left(val) => self.with_heading(*val),
            Action::Right(val) => self.with_heading(-val),
            Action::Forward(val) => {
                if self.heading == 0 {
                    self.with_x(*val)
                } else if self.heading == 90 || self.heading == -270 {
                    self.with_y(*val)
                } else if self.heading == 180 || self.heading == -180 {
                    self.with_x(-val)
                } else if self.heading == 270 || self.heading == -90 {
                    self.with_y(-val)
                } else {
                    panic!("Unhandled heading of {}", self.heading);
                }
            }
        }
    }

    pub fn apply_part2(&self, action: &Action) -> Self {
        match action {
            Action::North(val) => self.with_waypoint_y(*val),
            Action::South(val) => self.with_waypoint_y(-val),
            Action::East(val) => self.with_waypoint_x(*val),
            Action::West(val) => self.with_waypoint_x(-val),
            Action::Left(val) => self.with_waypoint_rotate(*val),
            Action::Right(val) => self.with_waypoint_rotate(-val),
            Action::Forward(val) => self
                .with_x(val * self.waypoint.x)
                .with_y(val * self.waypoint.y),
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
        let mut iter = script.iter();
        ship = ship.apply(iter.next().unwrap());
        assert_eq!(10, ship.x);
        assert_eq!(0, ship.y);
        assert_eq!(0, ship.heading);

        ship = ship.apply(iter.next().unwrap());
        assert_eq!(10, ship.x);
        assert_eq!(3, ship.y);
        assert_eq!(0, ship.heading);

        ship = ship.apply(iter.next().unwrap());
        assert_eq!(17, ship.x);
        assert_eq!(3, ship.y);
        assert_eq!(0, ship.heading);

        ship = ship.apply(iter.next().unwrap());
        assert_eq!(17, ship.x);
        assert_eq!(3, ship.y);
        assert_eq!(-90, ship.heading);

        ship = ship.apply(iter.next().unwrap());
        assert_eq!(17, ship.x);
        assert_eq!(-8, ship.y);
        assert_eq!(-90, ship.heading);

        assert_eq!(25, ship.manhattan_distance());

        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        let script = Action::parse_script_from_string(
            "F10
N3
F7
R90
F11",
        )?;
        let mut ship = Ship::new();
        let mut iter = script.iter();
        ship = ship.apply_part2(iter.next().unwrap());
        assert_eq!(100, ship.x);
        assert_eq!(10, ship.y);
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(1, ship.waypoint.y);

        ship = ship.apply_part2(iter.next().unwrap());
        assert_eq!(100, ship.x);
        assert_eq!(10, ship.y);
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(4, ship.waypoint.y);

        ship = ship.apply_part2(iter.next().unwrap());
        assert_eq!(170, ship.x);
        assert_eq!(38, ship.y);
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(4, ship.waypoint.y);

        ship = ship.apply_part2(iter.next().unwrap());
        assert_eq!(170, ship.x);
        assert_eq!(38, ship.y);
        assert_eq!(4, ship.waypoint.x);
        assert_eq!(-10, ship.waypoint.y);

        ship = ship.apply_part2(iter.next().unwrap());
        assert_eq!(214, ship.x);
        assert_eq!(-72, ship.y);
        assert_eq!(4, ship.waypoint.x);
        assert_eq!(-10, ship.waypoint.y);

        assert_eq!(286, ship.manhattan_distance());

        Ok(())
    }

    #[test]
    fn north_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::North(1));
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(2, ship.waypoint.y);
    }

    #[test]
    fn south_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(0, ship.waypoint.y);
    }

    #[test]
    fn east_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::East(1));
        assert_eq!(11, ship.waypoint.x);
        assert_eq!(1, ship.waypoint.y);
    }

    #[test]
    fn west_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::West(1));
        assert_eq!(9, ship.waypoint.x);
        assert_eq!(1, ship.waypoint.y);
    }

    #[test]
    fn left90_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Left(90));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(10, ship.waypoint.y);
    }

    #[test]
    fn left180_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Left(180));
        assert_eq!(-10, ship.waypoint.x);
        assert_eq!(0, ship.waypoint.y);
    }

    #[test]
    fn left270_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Left(270));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(-10, ship.waypoint.y);
    }

    #[test]
    fn right90_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Right(90));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(-10, ship.waypoint.y);
    }

    #[test]
    fn right180_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Right(180));
        assert_eq!(-10, ship.waypoint.x);
        assert_eq!(0, ship.waypoint.y);
    }

    #[test]
    fn right270_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        ship = ship.apply_part2(&Action::Right(270));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(10, ship.waypoint.y);
    }

    #[test]
    fn forward_part2() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::Forward(7));
        assert_eq!(70, ship.x);
        assert_eq!(7, ship.y);
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(1, ship.waypoint.y);
    }

    #[test]
    fn rotate() {
        let mut ship = Ship::new();
        ship = ship.apply_part2(&Action::South(1));
        assert_eq!(10, ship.waypoint.x);
        assert_eq!(0, ship.waypoint.y);

        ship = ship.apply_part2(&Action::Left(90));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(10, ship.waypoint.y);

        ship = ship.apply_part2(&Action::Right(180));
        assert_eq!(0, ship.waypoint.x);
        assert_eq!(-10, ship.waypoint.y);

        ship = ship.apply_part2(&Action::Left(270));
        assert_eq!(-10, ship.waypoint.x);
        assert_eq!(0, ship.waypoint.y);
    }
}