use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let maze = Maze::parse(&contents);

    let minsteps = maze.count_min_steps();

    println!("The shortest path is {}.", minsteps);
}

struct Maze {
    start: Point,
    end: Point,
    walls: HashSet<Point>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };
        let mut walls = HashSet::new();

        let mut y = 0;
        for line in input.lines() {
            let mut x = 0;
            for ch in line.chars() {
                match ch {
                    '#' => {
                        walls.insert(Point { x, y });
                    }
                    'S' => start = Point { x, y },
                    'E' => end = Point { x, y },
                    '.' => {}
                    _ => unreachable!(),
                }
                x += 1;
            }
            y += 1;
        }
        Self { start, end, walls }
    }

    fn count_min_steps(&self) -> usize {
        let mut seen = HashMap::<State, usize>::new();
        let mut queue = VecDeque::new();
        queue.push_back(StateWithCost {
            state: State {
                position: self.start,
                direction: Direction::East,
            },
            cost: 0,
        });

        let mut min = usize::MAX;
        while !queue.is_empty() {
            let s = queue.pop_front().unwrap();
            //println!("Considering {:?}", s);
            if s.state.position == self.end {
                if s.cost < min {
                    min = s.cost;
                }
            } else {
                for n in self.next_positions(&s) {
                    if let Some(existing) = seen.get(&n.state) {
                        if *existing > n.cost {
                            seen.insert(n.state, n.cost);
                            queue.push_back(n);
                        }
                    } else {
                        seen.insert(n.state, n.cost);
                        queue.push_back(n);
                    }
                }
            }
        }

        min
    }


    fn next_positions(&self, state: &StateWithCost) -> Vec<StateWithCost> {
        let mut res = Vec::new();

        let advanced = state.advance();
        if !self.walls.contains(&advanced.state.position) {
            
            res.push(advanced);
        }

        res.push(state.turn_right());
        res.push(state.turn_left());
        res
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    position: Point,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct StateWithCost {
    state: State,
    cost: usize,
}

impl StateWithCost {
    fn advance(&self) -> Self {
        let position = match self.state.direction {
            Direction::East => Point {
                x: self.state.position.x + 1,
                y: self.state.position.y,
            },
            Direction::North => Point {
                x: self.state.position.x,
                y: self.state.position.y - 1,
            },
            Direction::South => Point {
                x: self.state.position.x,
                y: self.state.position.y + 1,
            },
            Direction::West => Point {
                x: self.state.position.x - 1,
                y: self.state.position.y,
            },
        };
        Self {
            state: State {
                position,
                direction: self.state.direction,
            },
            cost: self.cost + 1,
        }
    }

    fn turn_right(&self) -> Self {
        let direction = match self.state.direction {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            state: State {
                position: self.state.position,
                direction: direction,
            },
            cost: self.cost + 1000,
        }
    }

    fn turn_left(&self) -> Self {
        let direction = match self.state.direction {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            state: State {
                position: self.state.position,
                direction: direction,
            },
            cost: self.cost + 1000,
        }
    }
}
