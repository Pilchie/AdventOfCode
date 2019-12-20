use intcode;
use std::collections::HashMap;
use std::env;
use std::fs;

enum ExpectedOutput {
    Color,
    Turn,
}

impl ExpectedOutput {
    fn next(&self) -> ExpectedOutput {
        match self {
            ExpectedOutput::Color => ExpectedOutput::Turn,
            ExpectedOutput::Turn => ExpectedOutput::Color,
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn next(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                y: self.y - 1,
                ..*self
            },
            Direction::Right => Position {
                x: self.x + 1,
                ..*self
            },
            Direction::Down => Position {
                y: self.y + 1,
                ..*self
            },
            Direction::Left => Position {
                x: self.x - 1,
                ..*self
            },
        }
    }
}

impl Clone for Position {
    fn clone(&self) -> Position {
        Position { ..*self }
    }
}
impl Copy for Position {}

impl std::cmp::PartialEq for Position {
    fn eq(&self, rhs: &Position) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl std::cmp::Eq for Position {}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        *self
    }
}

impl Copy for Direction {}

impl Direction {
    fn next(&self, val: i64) -> Direction {
        match (self, val) {
            (Direction::Up, 0) => Direction::Left,
            (Direction::Up, 1) => Direction::Right,

            (Direction::Right, 0) => Direction::Up,
            (Direction::Right, 1) => Direction::Down,

            (Direction::Down, 0) => Direction::Right,
            (Direction::Down, 1) => Direction::Left,

            (Direction::Left, 0) => Direction::Down,
            (Direction::Left, 1) => Direction::Up,

            _ => panic!("Unexpected direction transition"),
        }
    }
}

enum Color {
    Black,
    White,
}

struct Robot {
    grid: HashMap<Position, Color>,
    pos: Position,
    dir: Direction,
    expected: ExpectedOutput,
}

impl Robot {
    fn new() -> Robot {
        let mut robot = Robot {
            grid: HashMap::new(),
            pos: Position { x: 0, y: 0 },
            dir: Direction::Up,
            expected: ExpectedOutput::Color,
        };
        robot.grid.insert(Position { x: 0, y: 0 }, Color::White);
        robot
    }
}

impl intcode::InputOutputSystem for Robot {
    fn get_input(&mut self) -> i64 {
        let value = self.grid.entry(self.pos).or_insert(Color::Black);
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn print_output(&mut self, value: i64) {
        match self.expected {
            ExpectedOutput::Color => {
                if value == 1 {
                    self.grid.insert(self.pos, Color::White);
                } else {
                    self.grid.insert(self.pos, Color::Black);
                }
            }
            ExpectedOutput::Turn => {
                self.dir = self.dir.next(value);
                self.pos = self.pos.next(self.dir);
            }
        }

        self.expected = self.expected.next();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut input: Vec<i64> = Vec::new();
    for i in contents.split(',') {
        input.push(i.parse::<i64>().unwrap());
    }

    let mut robot = Robot::new();
    let mut computer = intcode::IntCode::new(&mut robot);
    computer.run_to_completion(&mut input);

    let mut minx: Option<i32> = None;
    let mut miny: Option<i32> = None;
    let mut maxx: Option<i32> = None;
    let mut maxy: Option<i32> = None;

    for p in robot.grid.keys() {
        minx = match minx {
            None => Some(p.x),
            Some(min) => {
                if p.x < min {
                    Some(p.x)
                } else {
                    minx
                }
            }
        };
        miny = match miny {
            None => Some(p.y),
            Some(min) => {
                if p.y < min {
                    Some(p.y)
                } else {
                    miny
                }
            }
        };
        maxx = match maxx {
            None => Some(p.x),
            Some(max) => {
                if p.x > max {
                    Some(p.x)
                } else {
                    maxx
                }
            }
        };
        maxy = match maxy {
            None => Some(p.y),
            Some(max) => {
                if p.y > max {
                    Some(p.y)
                } else {
                    maxy
                }
            }
        };
    }

    for y in miny.unwrap()..(maxy.unwrap() + 1) {
        for x in minx.unwrap()..(maxx.unwrap() + 1) {
            let color = &robot.grid.entry(Position { x, y }).or_insert(Color::Black);
            match color {
                Color::White => print!("█"),
                Color::Black => print!(" "),
            }
        }
        println!();
    }

    println!("{}", robot.grid.len());
}
