use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
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

    let mut map = Octopuses::parse(&input);
    let mut step = 0;
    loop {
        step += 1;
        map.step();

        if map.all_flashed() {
            break;
        }
    }

    println!("There first synchronized flash was at step {}", step);

    Ok(())
}

struct Octopuses {
    points: Vec<Vec<i32>>,
}

impl Octopuses {
    fn parse(input: &str) -> Self {
        let mut rows = Vec::new();
        let a = "0".bytes().next().unwrap();
        for line in input.lines() {
            let row: Vec<_> = line.bytes().map(|b| (b - a) as i32).collect();
            rows.push(row);
        }

        Self { points: rows }
    }

    fn step(self: &mut Self) -> usize {
        let mut flashes = HashSet::new();

        for y in 0..self.points.len() {
            for x in 0..self.points[y].len() {
                self.points[y][x] += 1;
                if self.points[y][x] == 10 {
                    flashes.insert(Point { y, x });
                }
            }
        }

        let mut flashed = HashSet::new();
        while flashes.len() != flashed.len() {
            self.flash(&mut flashes, &mut flashed);
        }

        for y in 0..self.points.len() {
            for x in 0..self.points[y].len() {
                if self.points[y][x] > 9 {
                    self.points[y][x] = 0;
                }
            }
        }

        flashes.len()
    }

    fn flash(self: &mut Self, flashes: &mut HashSet<Point>, flashed: &mut HashSet<Point>) {
        let mut new: HashSet<Point> = HashSet::new();
        for p in flashes.iter() {
            if !flashed.contains(p) {
                flashed.insert(*p);
                for pa in self.adjacent(p) {
                    self.points[pa.y][pa.x] += 1;
                    if self.points[pa.y][pa.x] == 10 {
                        new.insert(Point { y: pa.y, x: pa.x });
                    }
                }
            }
        }

        flashes.extend(new);
    }

    fn adjacent(self: &Self, p: &Point) -> Vec<Point> {
        let mut res = Vec::new();

        // above left
        if p.x > 0 && p.y > 0 {
            res.push(Point { y: p.y - 1, x: p.x - 1, });
        }

        // above
        if p.y > 0 {
            res.push(Point { y: p.y - 1, x: p.x });
        }

        // above right
        if p.y > 0 && p.x < self.points[p.y].len() - 1 {
            res.push(Point { y: p.y - 1, x: p.x + 1, });
        }

        // left
        if p.x > 0  {
            res.push(Point { y: p.y, x: p.x - 1, });
        }

        // right
        if p.x < self.points[p.y].len() - 1 {
            res.push(Point { y: p.y, x: p.x + 1, });
        }

        // below left
        if p.x > 0 && p.y < self.points.len() - 1 {
            res.push(Point { y: p.y + 1, x: p.x - 1, });
        }

        // below
        if p.y < self.points.len() - 1 {
            res.push(Point { y: p.y + 1, x: p.x });
        }

        // below right
        if p.y < self.points.len() - 1 && p.x < self.points[p.y].len() - 1 {
            res.push(Point { y: p.y + 1, x: p.x + 1, });
        }
        
        res
    }

    fn all_flashed(self: &Self) -> bool {
        for row in &self.points {
            for p in row {
                if *p != 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
