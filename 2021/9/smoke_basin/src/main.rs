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

    let map = HeightMap::parse(&input);
    let mut sizes = Vec::new();

    for y in 0..map.points.len() {
        for x in 0..map.points[y].len() {
            if map.is_low_point(x, y) {
                let size = map.find_basin_size(x, y);
                sizes.push(size);
                println!("Found low point at ({},{}) with basin size {}.", y, x, size);
            }
        }
    }

    sizes.sort();
    sizes.reverse();
    let mut res = 1;

    for (i, s) in sizes.iter().enumerate() {
        if i > 2 {
            break;
        }

        res *= s;
    }

    println!("The product of the basin sizes is {}", res);

    Ok(())
}

struct HeightMap {
    points: Vec<Vec<u8>>,
}

impl HeightMap {
    fn parse(input: &str) -> Self {
        let mut res = Vec::new();
        let a = "0".bytes().next().unwrap();
        for line in input.lines() {
            let row: Vec<_> = line.bytes().map(|b| b - a).collect();
            res.push(row);
        }
        Self { points: res }
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
        if y < self.points.len() - 1 && val > self.points[y + 1][x] {
            return false;
        }

        // right
        if x < self.points[y].len() - 1 && val > self.points[y][x + 1] {
            return false;
        }

        true
    }

    fn _risk_level(self: &Self, x: usize, y: usize) -> u32 {
        (self.points[y][x] + 1).into()
    }

    fn find_basin_size(self: &Self, x: usize, y: usize) -> usize {
        let mut explored = HashSet::new();
        let mut basin = HashSet::new();
        let p = Point { x, y };
        basin.insert(p);

        while basin.len() != explored.len() {
            self.explore_basin(&mut basin, &mut explored)
        }

        basin.len()
    }

    fn explore_basin(self: &Self, basin: &mut HashSet<Point>, explored: &mut HashSet<Point>) {
        let mut new = HashSet::new();
        for p in basin.iter() {
            if !explored.contains(p) {
                explored.insert(*p);

                // left
                for x in 1..p.x + 1 {
                    //print!("Checking ({},{})...", p.y, p.x - x);
                    if p.x >= x && self.points[p.y][p.x - x] < 9 {
                        //println!("Added");
                        new.insert(Point { x: p.x - x, y: p.y });
                    } else {
                        //println!("Stopping");
                        break;
                    }
                }

                // right
                for x in (p.x + 1)..self.points[p.y].len() {
                    //print!("Checking ({},{})...", p.y, x);
                    if x < self.points[p.y].len() && self.points[p.y][x] < 9 {
                        //println!("Added");
                        new.insert(Point { x: x, y: p.y });
                    } else {
                        //println!("Stopping");
                        break;
                    }
                }

                // up
                for y in 1..p.y + 1 {
                    //print!("Checking ({},{})...", p.y - y, p.x);
                    if p.y >= y && self.points[p.y - y][p.x] < 9 {
                        //println!("Added");
                        new.insert(Point { x: p.x, y: p.y - y });
                    } else {
                        //println!("Stopping");
                        break;
                    }
                }

                // down
                for y in (p.y + 1)..self.points.len() {
                    //print!("Checking ({},{})...", y, p.x);
                    if y < self.points.len() && self.points[y][p.x] < 9 {
                        //println!("Added");
                        new.insert(Point { x: p.x, y: y });
                    } else {
                        //println!("Stopping");
                        break;
                    }
                }
            }
        }

        basin.extend(new);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
