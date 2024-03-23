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
        vents.push(v);
    }

    let mut count_with_two = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            let p = Point { x, y };
            let mut count_this_point = 0;
            for v in &vents {
                if v.intersects(&p) {
                    //println!("{:?} intersects {:?}", p, v);
                    count_this_point += 1;
                }

                if count_this_point == 2 {
                    count_with_two += 1;
                    break;
                }
            }
            print!("{}", count_this_point);
        }
        println!();
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

    fn _is_diagonal(self: &Self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn intersects(self: &Self, point: &Point) -> bool {
        if self.start.x == self.end.x {
            if self.start.x == point.x {
                if point.y >= self.start.y && point.y <= self.end.y
                    || point.y <= self.start.y && point.y >= self.end.y
                {
                    return true;
                }
            }
        } else if self.start.y == self.end.y {
            if self.start.y == point.y {
                if point.x >= self.start.x && point.x <= self.end.x
                    || point.x <= self.start.x && point.x >= self.end.x
                {
                    return true;
                }
            }
        } else if self.start.x < self.end.x {
            let length = self.end.x - self.start.x + 1;
            if self.start.y < self.end.y {
                for i in 0..length {
                    if point.x == self.start.x + i && point.y == self.start.y + i {
                        return true;
                    }
                }
            } else {
                for i in 0..length {
                    if point.x == self.start.x + i && point.y == self.start.y - i {
                        return true;
                    }
                }
            }
        } else if self.end.x < self.start.x {
            let length = self.start.x - self.end.x + 1;
            if self.start.y < self.end.y {
                for i in 0..length {
                    if point.x == self.start.x - i && point.y == self.start.y + i {
                        return true;
                    }
                }
            } else {
                for i in 0..length {
                    if point.x == self.start.x - i && point.y == self.start.y - i {
                        return true;
                    }
                }
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
