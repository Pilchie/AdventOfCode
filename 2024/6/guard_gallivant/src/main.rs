use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let (guard, map) = Map::parse(&contents);

    // Part 1
    {
        let mut visited = HashSet::new();
        let mut count = 0;

        visited.insert(guard.position);

        println!("starting at {:?}", guard);
        let mut guard = guard.clone();
        while map.contains(guard.position) {
            guard = map.advance(&guard);
            if !visited.contains(&guard.position) {
                visited.insert(guard.position);
                count += 1;
            }
        }
        println!("Done at {:?} count {}", guard, count);
    }

    // Part 2
    let mut obstacles_causing_loops = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point::new(x, y);
            if guard.position != p && !map.obstacles.contains(&p) {
                let map = map.place_obstacle(&p);
                let mut guard = guard.clone();
                let mut visited = HashSet::new();
                let mut count = 1;
                while map.contains(guard.position) && !visited.contains(&guard) {
                    visited.insert(guard.clone());
                    guard = map.advance(&guard);
                    count += 1;
                }
                if visited.contains(&guard) {
                    println!("Found loop with obstacle at {:?}, moves: {}", p, count);
                    obstacles_causing_loops += 1;
                } else {
                }
            }
        }
    }
    println!(
        "Found {} positions to place an obstacle to cause a loop",
        obstacles_causing_loops
    );
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            height += 1;
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

        println!(
            "Constructed a {} x {} map with {} obstacles and guard starting at {:?}",
            width,
            height,
            m.obstacles.len(),
            g
        );

        (g, m)
    }

    fn contains(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }

    fn advance(&self, guard: &Guard) -> Guard {
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
            return Guard {
                position: guard.position,
                direction: guard.direction.next(),
            };
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

    fn place_obstacle(&self, o: &Point) -> Self {
        let mut obstacles = self.obstacles.clone();
        obstacles.push(*o);
        Self {
            obstacles: obstacles,
            width: self.width,
            height: self.height,
        }
    }
}
