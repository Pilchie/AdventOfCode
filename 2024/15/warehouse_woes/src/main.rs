use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut warehouse = Warehouse::parse(&contents);
    println!("There are {} boxes to start", warehouse.count_boxes());
    let mut start = false;
    // println!("Initial state:");
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
    println!("There are {} boxes to finish", warehouse.count_boxes());
    println!("The sum of gps coords is {}", warehouse.sum_gps());
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Empty,
    Wall,
    BoxL,
    BoxR,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
                    'O' => State::BoxL,
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
                if s == State::BoxL {
                    row.push(State::BoxR);
                } else {
                    row.push(s);
                }
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
                if self.state[r][c] == State::BoxL {
                    sum += 100 * r + c;
                }
            }
        }

        sum
    }

    fn try_move_up(&self, copied: &mut Vec<Vec<State>>) -> Point {
        let mut possible = false;
        let mut blocked = false;
        let mut boxes_to_push = Vec::new();

        let mut boxes_above = HashSet::new();
        match self.state[self.robot.y - 1][self.robot.x] {
            State::Empty => possible = true,
            State::Wall => blocked = true,
            State::BoxL => {
                boxes_above.insert(Point {
                    y: self.robot.y - 1,
                    x: self.robot.x,
                });
            }
            State::BoxR => {
                boxes_above.insert(Point {
                    y: self.robot.y - 1,
                    x: self.robot.x - 1,
                });
            }
        };
        if !boxes_above.is_empty() {
            boxes_to_push.push(boxes_above.clone());
        }

        while !blocked && !possible {
            let mut nextrow = HashSet::new();

            for b in &boxes_above {
                match self.state[b.y - 1][b.x] {
                    State::Empty => {}
                    State::Wall => blocked = true,
                    State::BoxL => {
                        nextrow.insert(Point { y: b.y - 1, x: b.x });
                    }
                    State::BoxR => {
                        nextrow.insert(Point {
                            y: b.y - 1,
                            x: b.x - 1,
                        });
                    }
                };
                match self.state[b.y - 1][b.x + 1] {
                    State::Empty => {}
                    State::Wall => blocked = true,
                    State::BoxL => {
                        nextrow.insert(Point {
                            y: b.y - 1,
                            x: b.x + 1,
                        });
                    }
                    State::BoxR => {
                        // Do nothing, we would have already added this in the match above.
                    }
                };
            }

            if nextrow.is_empty() {
                if !blocked {
                    possible = true;
                }
            } else {
                boxes_to_push.push(nextrow.clone());
                boxes_above = nextrow;
            }
        }

        if possible {
            boxes_to_push.reverse();
            for row in &boxes_to_push {
                for b in row {
                    copied[b.y - 1][b.x] = copied[b.y][b.x];
                    copied[b.y - 1][b.x + 1] = copied[b.y][b.x + 1];
                    copied[b.y][b.x] = State::Empty;
                    copied[b.y][b.x + 1] = State::Empty;
                }
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
        let mut blocked = false;
        let mut boxes_to_push = Vec::new();

        let mut boxes_below = HashSet::new();
        match self.state[self.robot.y + 1][self.robot.x] {
            State::Empty => possible = true,
            State::Wall => blocked = true,
            State::BoxL => {
                boxes_below.insert(Point {
                    y: self.robot.y + 1,
                    x: self.robot.x,
                });
            }
            State::BoxR => {
                boxes_below.insert(Point {
                    y: self.robot.y + 1,
                    x: self.robot.x - 1,
                });
            }
        };
        if !boxes_below.is_empty() {
            boxes_to_push.push(boxes_below.clone());
        }

        while !blocked && !possible {
            let mut nextrow = HashSet::new();

            for b in &boxes_below {
                match self.state[b.y + 1][b.x] {
                    State::Empty => {}
                    State::Wall => blocked = true,
                    State::BoxL => {
                        nextrow.insert(Point { y: b.y + 1, x: b.x });
                    }
                    State::BoxR => {
                        nextrow.insert(Point {
                            y: b.y + 1,
                            x: b.x - 1,
                        });
                    }
                };
                match self.state[b.y + 1][b.x + 1] {
                    State::Empty => {}
                    State::Wall => blocked = true,
                    State::BoxL => {
                        nextrow.insert(Point {
                            y: b.y + 1,
                            x: b.x + 1,
                        });
                    }
                    State::BoxR => {
                        // Do nothing, we would have already added this in the match above.
                    }
                };
            }

            if nextrow.is_empty() {
                if !blocked {
                    possible = true;
                }
            } else {
                boxes_to_push.push(nextrow.clone());
                boxes_below = nextrow;
            }
        }

        if possible {
            boxes_to_push.reverse();
            for row in &boxes_to_push {
                for b in row {
                    copied[b.y + 1][b.x] = copied[b.y][b.x];
                    copied[b.y + 1][b.x + 1] = copied[b.y][b.x + 1];
                    copied[b.y][b.x] = State::Empty;
                    copied[b.y][b.x + 1] = State::Empty;
                }
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
                State::BoxR => x = x - 1,
                State::Wall => atwall = true,
                State::BoxL => x = x - 1,
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
                State::BoxL => x = x + 1,
                State::Wall => atwall = true,
                State::BoxR => x = x + 1,
            }
        }

        if possible {
            let mut xi = x - 1;
            while xi >= self.robot.x + 1 {
                copied[self.robot.y][xi + 1] = copied[self.robot.y][xi].clone();
                xi -= 1;
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
                        State::BoxL => print!("["),
                        State::BoxR => print!("]"),
                        State::Wall => print!("#"),
                        State::Empty => print!("."),
                    };
                }
            }
            println!();
        }
    }

    fn count_boxes(&self) -> usize {
        let mut count = 0;
        for r in &self.state {
            for s in r {
                if *s == State::BoxL {
                    count += 1;
                }
            }
        }
        count
    }
}
