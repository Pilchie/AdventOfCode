use intcode;
use std::collections::HashMap;

enum ExpectedOutput {
    Color,
    Turn,
}

impl ExpectedOutput {
    fn Next(&self) -> ExpectedOutput {
        match self {
            ExpectedOutput::Color => ExpectedOutput::Turn,
            ExpectedOutput::Turn => ExpectedOutput::Color,
        }
    }
}

struct Position {
    x: usize,
    y: usize,
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
        state.write_usize(self.x);
        state.write_usize(self.y);
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
    grid: Vec<Vec<Color>>,
    pos: Position,
    dir: Direction,
    expected: ExpectedOutput,
    painted: HashMap<Position, bool>,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            grid: vec![
                vec![
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                ],
                vec![
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                ],
                vec![
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                ],
                vec![
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                ],
                vec![
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                    Color::Black,
                ],
            ],
            pos: Position { x: 0, y: 0 },
            dir: Direction::Up,
            expected: ExpectedOutput::Color,
            painted: HashMap::new(),
        }
    }
}

impl intcode::InputProvider for Robot {
    fn get_input(&self) -> i64 {
        match self.grid[self.pos.y][self.pos.x] {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl intcode::OutputSink for Robot {
    fn print_output(&mut self, value: i64) {
        match self.expected {
            ExpectedOutput::Color => {
                self.painted.insert(self.pos, true);
                if value == 1 {
                    self.grid[self.pos.y][self.pos.x] = Color::White;
                } else {
                    self.grid[self.pos.y][self.pos.x] = Color::Black;
                }
            }
            ExpectedOutput::Turn => {
                self.dir = self.dir.next(value);
                self.pos = self.pos.next(self.dir);
            }
        }

        self.expected = self.expected.Next();
    }
}

fn main() {
    let mut robot = Robot::new();
    let mut computer = intcode::IntCode::new(&robot, &mut robot);
    let mut input = vec![1, 0, 0, 0, 99];
    computer.run_to_completion(&mut input);

    println!("{}", robot.painted.len());
}
