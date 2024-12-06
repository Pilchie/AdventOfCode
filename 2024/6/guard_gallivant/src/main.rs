use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut count = 0;
    let (mut guard, map) = Map::parse(&contents);

    while map.contains(guard.position) {
        guard = map.advance(guard);
        count += 1;
    }

    println!("count {}", count);
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone)]
struct Guard {
    position: Point,
    direction: Direction,
}

struct Map {
    obstacles: Vec<Point>,
    width: i32,
    height: i32,
}

impl Map {
    fn parse(map: &str) -> (Guard, Self) {
        let mut width = 0;
        let mut height = 0;
        let mut obstacles = Vec::new();
        let mut pos = Point::default();
        for line in map.lines() {
            width = 0;

            for ch in line.chars() {
                if ch == '#' {
                    obstacles.push(Point::new(width, height));
                } else if ch == '^' {
                    pos = Point::new(width, height);
                }
                width += 1;

            }
            height+= 1;
        }
        let g = Guard {
            position: pos,
            direction: Direction::Up,
        };
        let m = Self {
            obstacles,
            width,
            height,
        };

        (g, m)
    }

    fn contains(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.y >= 0 || pos.x < self.width || pos.y < self.height
    }

    fn advance(&self, guard: Guard) -> Guard {
        let mut guard = guard.clone();
        loop {
            let newpos = Self::forward(&guard);
            if !self.contains(newpos) {
                return Guard {
                    position: newpos,
                    direction: guard.direction,
                };
            }
            if !self.obstacles.contains(&newpos) {
                return Guard {
                    position: newpos,
                    direction: guard.direction,
                };
            } else {
                guard = Guard {
                    position: guard.position,
                    direction: guard.direction.next(),
                };
            }
        }
    }

    fn forward(guard: &Guard) -> Point {
        match guard.direction {
            Direction::Up => Point {
                x: guard.position.x,
                y: guard.position.y - 1,
            },
            Direction::Right => Point {
                x: guard.position.x + 1,
                y: guard.position.y,
            },
            Direction::Down => Point {
                x: guard.position.x,
                y: guard.position.y + 1,
            },
            Direction::Left => Point {
                x: guard.position.x - 1,
                y: guard.position.y,
            },
        }
    }
}
