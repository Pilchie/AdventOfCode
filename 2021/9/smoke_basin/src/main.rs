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
    let input = std::fs::read_to_string(&args[1])?;

    let map = HeightMap::parse(&input);

    let mut res = 0;

    for y in 0..map.points.len() {
        for x in 0..map.points[y].len() {
            if map.is_low_point(x, y) {
                let risk = map.risk_level(x, y);
                println!("Found low point at ({},{}) with risk {}", y, x, risk);
                res += risk;
            }
        }
    }

    println!("The sum of risk levels of low points is {}", res);

    Ok(())
}

struct HeightMap {
    points: Vec<Vec<u8>>,
}

impl HeightMap {
    fn parse(input: &str) -> Self {
        let mut res = Vec::new();
        for line in input.lines() {
            let row : Vec<_> = line.bytes().collect();
            res.push(row);
        }
        Self {
            points: res,
        }
    }

    fn is_low_point(self: &Self, x: usize, y: usize) -> bool {
        let val = self.points[y][x];

        // above
        if y > 0 && val >= self.points[y - 1][x] {
            return false;
        }

        // left
        if x > 0 && val >= self.points[y][x - 1] {
            return false;
        }

        // below
        if y < self.points.len() - 1 && val > self.points[y+1][x] {
            return false;
        }

        // right
        if x < self.points[y].len() - 1 && val > self.points[y][x + 1] {
            return false;
        }

        true
    }

    fn risk_level(self: &Self, x: usize, y: usize) -> u32 {
        let a = "0".bytes().next().unwrap();
        (self.points[y][x] - a + 1).into()
    }
}