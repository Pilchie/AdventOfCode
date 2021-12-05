use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::cmp::{max, min};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut m = Mosaic::new(&input);
    for _ in 0..100 {
        m = m.next_day();
    }
    println!("{}", m.black_tiles.len());

    Ok(())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    pub fn delta(direction: &Direction) -> Self {
        match direction {
            Direction::East => Point::new(2, 0),
            Direction::SouthEast => Point::new(1, -1),
            Direction::SouthWest => Point::new(-1, -1),
            Direction::West => Point::new(-2, 0),
            Direction::NorthWest => Point::new(-1, 1),
            Direction::NorthEast => Point::new(1, 1),

        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    pub fn parse(input: &str) -> Vec<Direction> {
        let chars:Vec<_> = input.chars().collect();
        let mut i = 0;
        let mut res = Vec::new();
        while i < chars.len() - 1 {
            res.push(match chars[i] {
                'e' => Direction::East,
                'w' => Direction::West,
                'n' => {
                    i += 1;
                    match chars[i] {
                        'e' => Direction::NorthEast,
                        'w' => Direction::NorthWest,
                        _ => panic!("Unexpected char {}", chars[i]),
                    }
                },
                's' => {
                    i += 1;
                    match chars[i] {
                        'e' => Direction::SouthEast,
                        'w' => Direction::SouthWest,
                        _ => panic!("Unexpected char {}", chars[i]),
                    }
                },
                _ => panic!("Unexpected char {}", chars[i]),
            });
            i += 1;
        }

        if i < chars.len() {
            let c = chars[chars.len() - 1];
            res.push(match c {
                'e' => Direction::East,
                'w' => Direction::West,
                _ => panic!("Unexpected char {}", c),
            })
        }
        res
    }

    pub fn resolve(directions: &[Direction]) -> Point {
        let mut res = Point::zero();
        for d in directions {
            res = res.add(&Point::delta(d));
        }
        res
    }
}

pub struct Mosaic {
    black_tiles: HashSet<Point>,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

impl Mosaic {
    pub fn new(input: &str) -> Self {
        let mut x_range = RangeInclusive::new(0, 0);
        let mut y_range = RangeInclusive::new(0, 0);

        let mut black = HashSet::new();
        for line in input.lines() {
            let p = Direction::resolve(&Direction::parse(line));
            x_range = RangeInclusive::new(min(*x_range.start(), p.x), max(*x_range.end(), p.x));
            y_range = RangeInclusive::new(min(*y_range.start(), p.y), max(*y_range.end(), p.y));
            if !black.remove(&p) {
                black.insert(p);
            }
        }

        Self {
            black_tiles: black,
            x_range: x_range,
            y_range: y_range,
        }
    }

    pub fn next_day(&self) -> Self {
        let mut new = self.black_tiles.clone();
        // extend one past each existing point to see if new black tiles
        // are created at the edges.
        for x in self.x_range.start()-1..self.x_range.end() + 2 {
            for y in self.y_range.start()-1..self.y_range.end() + 2 {
                let p = Point::new(x, y);
                let adjacent_black = self.count_adjacent(&p);
                if self.black_tiles.contains(&p) {
                    if adjacent_black == 0 || adjacent_black > 2 {
                        new.remove(&p);
                    }
                } else {
                    if adjacent_black == 2 {
                        new.insert(p);
                    }
                }
            }
        }

        // TODO: track whether we actually have any black tiles on the edge, and 
        // don't expand the range if not.
        Self {
            black_tiles: new,
            x_range: RangeInclusive::new(self.x_range.start() - 1, self.x_range.end() + 1),
            y_range: RangeInclusive::new(self.y_range.start() - 1, self.y_range.end() + 1)
        }
    }

    fn count_adjacent(&self, point: &Point) -> usize {
        let mut count = 0;
        if self.black_tiles.contains(&Point::new(point.x + 1, point.y + 1)) {
            count += 1;
        }
        if self.black_tiles.contains(&Point::new(point.x + 2, point.y)) {
            count += 1;
        }
        if self.black_tiles.contains(&Point::new(point.x + 1, point.y - 1)) {
            count += 1;
        }
        if self.black_tiles.contains(&Point::new(point.x - 1, point.y - 1)) {
            count += 1;
        }
        if self.black_tiles.contains(&Point::new(point.x - 2, point.y)) {
            count += 1;
        }
        if self.black_tiles.contains(&Point::new(point.x - 1, point.y + 1)) {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn parse() {
        let directions = Direction::parse("esenee");
        assert_all(&mut [
            Direction::East,
            Direction::SouthEast,
            Direction::NorthEast,
            Direction::East].iter(), &mut directions.iter());
    }

    #[test]
    fn coords_1() {
        let directions = Direction::parse("esenee");
        assert_eq!(Point::new(6, 0), Direction::resolve(&directions))
    }

    #[test]
    fn count() {
        assert_eq!(10, Mosaic::new("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew").black_tiles.len());
    }

    fn assert_all<I1, I2, T>(expected: &mut I1, actual: &mut I2)
        where I1: Iterator<Item=T>,
              I2: Iterator<Item=T>,
              T: std::fmt::Debug+PartialEq  {
        loop {
            if let Some(e) = expected.next() {
                match actual.next() {
                    Some(a) => assert_eq!(e, a),
                    None => assert!(false, format!("expected: '{:?}', actual end", e)),
                }
            } else {
                match actual.next() {
                    Some (a) => assert!(false, format!("expected end, actual: '{:?}'", a)),
                    None => return,
                }
            }
        }
    }
}


#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn days() {
        let mut m = Mosaic::new("sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew");
        assert_eq!(10, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(15, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(12, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(25, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(14, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(23, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(28, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(41, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(37, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(49, m.black_tiles.len());

        m = m.next_day();
        assert_eq!(37, m.black_tiles.len());

        let expecteds = [132, 259, 406, 566, 788, 1106, 1373, 1844, 2208];
        for x in &expecteds {
            for _ in 0..10 {
                m = m.next_day();
            }

            assert_eq!(x, &m.black_tiles.len());
        }
    }
}