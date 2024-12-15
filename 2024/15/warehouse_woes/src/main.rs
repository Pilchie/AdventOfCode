use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut warehouse = Warehouse::parse(&contents);

    let mut start = false;
//    println!("Initial state:");
    for line in contents.lines() {
        if line.is_empty() {
            start = true;
        } else if start {
            for ch in line.chars() {
                // warehouse.draw();
                // println!();
                // println!("Move {}:", ch);
                warehouse = warehouse.apply(ch);
            }
        }
    }
    println!("Final state");
    warehouse.draw();

    println!("The sum of gps coords is {}", warehouse.sum_gps());
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Empty,
    Wall,
    Box,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Warehouse {
    state: Vec<Vec<State>>,
    robot: Point,
}

impl Warehouse {
    fn parse(input: &str) -> Self {
        let mut rows = Vec::new();
        let mut robot = Point { x: 0, y: 0 };
        for line in input.lines() {
            if line.is_empty() {
                break;
            }

            let mut row = Vec::new();
            for ch in line.chars() {
                let s = match ch {
                    '#' => State::Wall,
                    '.' => State::Empty,
                    'O' => State::Box,
                    '@' => State::Empty,
                    _ => panic!("Unexpected char in map!"),
                };
                if ch == '@' {
                    robot = Point {
                        x: row.len(),
                        y: rows.len(),
                    };
                }
                row.push(s);
            }
            rows.push(row);
        }

        Self {
            state: rows,
            robot: robot,
        }
    }

    fn apply(&self, ch: char) -> Self {
        let mut copied = Vec::new();
        for r in &self.state {
            copied.push(r.clone());
        }

        let new_robot = match ch {
            '^' => self.try_move_up(&mut copied),
            'v' => self.try_move_down(&mut copied),
            '<' => self.try_move_left(&mut copied),
            '>' => self.try_move_right(&mut copied),
            _ => panic!("Unexpected instruction {}", ch),
        };

        Self {
            robot: new_robot,
            state: copied,
        }
    }

    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for r in 0..self.state.len() {
            for c in 0..self.state[r].len() {
                if self.state[r][c] == State::Box {
                    sum += 100 * r + c;
                }
            }
        }

        sum
    }

    fn try_move_up(&self, copied: &mut Vec<Vec<State>>) -> Point {
        let mut possible = false;
        let mut atwall = false;
        let mut y = self.robot.y - 1;
        while !atwall && !possible {
            match self.state[y][self.robot.x] {
                State::Empty => possible = true,
                State::Box => y -= 1,
                State::Wall => atwall = true,
            }
        }

        if possible {
            for yi in y..self.robot.y {
                copied[yi][self.robot.x] = copied[yi + 1][self.robot.x].clone();
            }
            copied[self.robot.y - 1][self.robot.x] = State::Empty;
        }

        match possible {
            true => Point {
                x: self.robot.x,
                y: self.robot.y - 1,
            },
            false => self.robot,
        }
    }

    fn try_move_down(&self, copied: &mut Vec<Vec<State>>) -> Point {
        let mut possible = false;
        let mut atwall = false;
        let mut y = self.robot.y + 1;
        while !atwall && !possible {
            match self.state[y][self.robot.x] {
                State::Empty => possible = true,
                State::Box => y = y + 1,
                State::Wall => atwall = true,
            }
        }

        if possible {
            for yi in self.robot.y + 1..y {
                copied[yi + 1][self.robot.x] = copied[yi][self.robot.x].clone();
            }
            copied[self.robot.y + 1][self.robot.x] = State::Empty;
        }

        match possible {
            true => Point {
                x: self.robot.x,
                y: self.robot.y + 1,
            },
            false => self.robot,
        }
    }

    fn try_move_left(&self, copied: &mut Vec<Vec<State>>) -> Point {
        let mut possible = false;
        let mut atwall = false;
        let mut x = self.robot.x - 1;
        while !atwall && !possible {
            match self.state[self.robot.y][x] {
                State::Empty => possible = true,
                State::Box => x = x - 1,
                State::Wall => atwall = true,
            }
        }

        if possible {
            for xi in x..self.robot.x {
                copied[self.robot.y][xi] = copied[self.robot.y][xi + 1].clone();
            }
            copied[self.robot.y][self.robot.x - 1] = State::Empty;
        }

        match possible {
            true => Point {
                x: self.robot.x - 1,
                y: self.robot.y,
            },
            false => self.robot,
        }
    }

    fn try_move_right(&self, copied: &mut Vec<Vec<State>>) -> Point {
        let mut possible = false;
        let mut atwall = false;
        let mut x = self.robot.x + 1;
        while !atwall && !possible {
            match self.state[self.robot.y][x] {
                State::Empty => possible = true,
                State::Box => x = x + 1,
                State::Wall => atwall = true,
            }
        }

        if possible {
            for xi in self.robot.x + 1..x {
                copied[self.robot.y][xi + 1] = copied[self.robot.y][xi].clone();
            }
            copied[self.robot.y][self.robot.x + 1] = State::Empty;
        }

        match possible {
            true => Point {
                x: self.robot.x + 1,
                y: self.robot.y,
            },
            false => self.robot,
        }
    }

    fn draw(&self) {
        for r in 0..self.state.len() {
            for c in 0..self.state[r].len() {
                if self.robot.y == r && self.robot.x == c {
                    print!("@");
                } else {
                    match self.state[r][c] {
                        State::Box => print!("O"),
                        State::Wall => print!("#"),
                        State::Empty => print!("."),
                    };
                }
            }
            println!();
        }
    }
}
