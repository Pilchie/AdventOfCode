use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let maze = Maze::parse(&contents);

    let best_paths = maze.best_paths();
    let mut tiles = HashSet::new();
    for path in &best_paths {
        for s in path {
            tiles.insert(s.state.position);
        }
    }

    println!(
        "The shortest path is {}, and there are {} spots along it.",
        best_paths.first().unwrap().last().unwrap().cost,
        tiles.len()
    );
    maze.print(&tiles);
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

    fn best_paths(&self) -> Vec<Vec<StateWithCost>> {
        let mut seen = HashMap::<State, usize>::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![StateWithCost {
            state: State {
                position: self.start,
                direction: Direction::East,
            },
            cost: 0,
        }]);

        let mut min = usize::MAX;
        let mut solutions = Vec::new();
        while !queue.is_empty() {
            let path = queue.pop_front().unwrap();
            let s = path.last().unwrap();
            // println!("Considering {:?}...", s);
            if s.state.position == self.end {
                if s.cost < min {
                    println!("  Found a new better solution with cost {}", s.cost);
                    min = s.cost;
                    solutions.clear();
                    solutions.push(path);
                } else if s.cost == min {
                    println!("  Adding an additional solution with cost {}", s.cost);
                    solutions.push(path);
                } else {
                    println!(
                        "  Found a solution, but not keeping, because it cost {}",
                        s.cost
                    );
                }
            } else if s.cost < min {
                for n in self.next_positions(&s) {
                    if let Some(existing) = seen.get(&n.state) {
                        if *existing >= n.cost {
                            let mut newpath = path.clone();
                            newpath.push(n);
                            queue.push_back(newpath);
                            // println!("  Updating with {:?} because its less than existing {}", n, existing);
                            seen.insert(n.state, n.cost);
                        } else {
                            // println!("  Not adding {:?}, because we we have cost {}", n, existing);
                        }
                    } else {
                        let mut newpath = path.clone();
                        newpath.push(n);
                        queue.push_back(newpath);
                        //  println!("  Adding {:?}", n);
                        seen.insert(n.state, n.cost);
                    }
                }
            } else {
                // println!("  Abandoning path as more than existing min");
            }
        }

        solutions
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

    fn print(&self, tiles: &HashSet<Point>) {
        let mut width = 0;
        let mut height = 0;
        for w in &self.walls {
            if w.x > width {
                width = w.x;
            }

            if w.y > height {
                height = w.y;
            }
        }

        for y in 0..height + 1 {
            for x in 0..width + 1 {
                let p = Point { x, y };
                if tiles.contains(&p) {
                    print!("O");
                } else if self.walls.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
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
