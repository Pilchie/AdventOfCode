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

    let mut vents = Vec::new();
    for l in reader.lines() {
        let line = l?;
        let v = Vector::parse(&line).ok_or(Error::Custom("can't parse vector".to_string()))?;
        if !v.is_diagonal() {
            vents.push(v);
        }
    }

    let mut count_with_two = 0;
    for x in 0..999 {
        for y in 0..999 {
            let p = Point{x,y,};
            let mut count_this_point = 0;
            for v in &vents {
                if v.intersects(&p) {
                    count_this_point += 1;
                }

                if count_this_point == 2 {
                    count_with_two += 1;
                    break;
                }
            }
        }
    }

    println!("There are {} points with two or more vents", count_with_two);

    Ok(())
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Vector {
    start: Point,
    end: Point,
}

impl Vector {
    fn parse(input: &str) -> Option<Self> {
        let (s, e) = input.split_once(" -> ")?;

        Some(Self {
            start: Point::parse(s)?,
            end: Point::parse(e)?,
        })
    }

    fn is_diagonal(self: &Self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn intersects(self: &Self, point: &Point) -> bool {
        if point.x == self.start.x && point.x == self.end.x {
            if point.y >= self.start.y && point.y <= self.end.y || point.y <= self.start.y && point.y >= self.end.y {
                return true;
            }
        } else if point.y == self.start.y && point.y == self.end.y {
            if point.x >= self.start.x && point.x <= self.end.x || point.x <= self.start.x && point.x >= self.end.x {
                return true;
            }
        }

        false
    }
}

impl Point {
    fn parse(input: &str) -> Option<Self> {
        let (x, y) = input.split_once(",")?;

        Some(Self {
            x: x.parse().ok()?,
            y: y.parse().ok()?,
        })
    }
}