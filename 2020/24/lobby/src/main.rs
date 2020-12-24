use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    println!("{}", Direction::count_black(&input));

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq)]
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

    pub fn count_black(input: &str) -> usize {
        let mut black = HashSet::new();
        for line in input.lines() {
            let p = Direction::resolve(&Direction::parse(line));
            if !black.remove(&p) {
                black.insert(p);
            }
        }
        black.len()
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
        assert_eq!(10, Direction::count_black("sesenwnenenewseeswwswswwnenewsewsw
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
wseweeenwnesenwwwswnew"));
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