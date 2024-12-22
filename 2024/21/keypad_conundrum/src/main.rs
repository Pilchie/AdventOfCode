use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    //let contents = "456A";

    let mut sum = 0;
    for line in contents.lines() {
        let num = &line[0..3].parse::<usize>().unwrap();
        let chars: Vec<_> = line.chars().collect();
        let first = DPad::solve(&chars, 'A', &NumPad::map());
        println!(" first: {:?}", String::from_iter(first.clone()));
        let second = DPad::solve(&first, 'A', &DPad::map());
        println!(" second: {:?}", String::from_iter(second.clone()));
        let human = DPad::solve(&second, 'A', &DPad::map());
        println!(" human: {:?}", String::from_iter(human.clone()));
        println!("Code {} took {} chars", line, human.len());
        let complexity = num * human.len();
        sum += complexity;
    }

    println!("The total complexity is {}", sum);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: Point,
    path: Vec<char>,
}

struct DPad {}

impl DPad {
    fn solve(code: &[char], start: char, map: &HashMap<Point, char>) -> Vec<char> {
        let mut res = Vec::new();
        let mut start_pos = Self::find(&start, map).unwrap();
        for ch in code {
            let end = Self::find(ch, map).unwrap();
            for ach in Self::solve_one(&start_pos, &end, map).unwrap() {
                res.push(ach);
            }
            start_pos = end;
            res.push('A');
        }
        res
    }

    fn find(ch: &char, map: &HashMap<Point, char>) -> Option<Point> {
        for (k, v) in map {
            if v == ch {
                return Some(*k);
            }
        }
        None
    }

    fn solve_one(start: &Point, end: &Point, map: &HashMap<Point, char>) -> Option<Vec<char>> {
        let mut queue = VecDeque::new();
        let mut shortest: Option<Vec<char>> = None;
        let mut seen: HashMap<Point, Vec<char>> = HashMap::new();

        queue.push_back(State {
            pos: *start,
            path: Vec::new(),
        });

        while !queue.is_empty() {
            let cur = queue.pop_back().unwrap();
            if cur.pos == *end {
                if Self::is_better(&cur, &shortest) {
                    shortest = Some(cur.path.clone());
                }
            }

            for n in Self::neighbors(&cur, &map) {
                if !seen.contains_key(&n.pos) || Self::is_better_path(&n, seen.get(&n.pos)?) {
                    seen.insert(n.pos, n.path.clone());
                    queue.push_back(n);
                }
            }
        }

        shortest
    }

    fn is_better(candidate: &State, existing: &Option<Vec<char>>) -> bool {
        if let Some(existing_path) = existing {
            return Self::is_better_path(candidate, existing_path);
        } else {
            return true;
        }
    }

    fn is_better_path(candidate: &State, existing_path: &[char]) -> bool {
        if candidate.path.len() < existing_path.len() {
            return true;
        } else if candidate.path.len() == existing_path.len() {
            if Self::count_turns(&candidate.path) < Self::count_turns(&existing_path) {
                return true;
            }
        }

        return false;
    }

    fn count_turns(path: &[char]) -> usize {
        let mut res = 0;
        for i in 1..path.len() {
            if path[i] != path[i - 1] {
                res += 1;
            }
        }
        res
    }

    fn neighbors(state: &State, map: &HashMap<Point, char>) -> Vec<State> {
        let mut res = Vec::new();
        let above = State {
            pos: Point {
                x: state.pos.x,
                y: state.pos.y - 1,
            },
            path: Self::with(&state.path, '^'),
        };
        let below = State {
            pos: Point {
                x: state.pos.x,
                y: state.pos.y + 1,
            },
            path: Self::with(&state.path, 'v'),
        };
        let right = State {
            pos: Point {
                x: state.pos.x + 1,
                y: state.pos.y,
            },
            path: Self::with(&state.path, '>'),
        };
        let left = State {
            pos: Point {
                x: state.pos.x - 1,
                y: state.pos.y,
            },
            path: Self::with(&state.path, '<'),
        };

        Self::addif(above, map, &mut res);
        Self::addif(left, map, &mut res);
        Self::addif(below, map, &mut res);
        Self::addif(right, map, &mut res);

        res
    }

    fn addif(state: State, map: &HashMap<Point, char>, res: &mut Vec<State>) {
        if map.contains_key(&state.pos) {
            res.push(state);
        }
    }

    fn with(vec: &[char], ch: char) -> Vec<char> {
        let mut res = vec.to_vec();
        res.push(ch);
        res
    }

    fn map() -> HashMap<Point, char> {
        let mut map = HashMap::new();
        map.insert(Point { x: 1, y: 0 }, '^');
        map.insert(Point { x: 2, y: 0 }, 'A');
        map.insert(Point { x: 0, y: 1 }, '<');
        map.insert(Point { x: 1, y: 1 }, 'v');
        map.insert(Point { x: 2, y: 1 }, '>');
        map
    }
}

struct NumPad {}

impl NumPad {
    fn map() -> HashMap<Point, char> {
        let mut map = HashMap::new();
        map.insert(Point { x: 1, y: 3 }, '0');
        map.insert(Point { x: 0, y: 2 }, '1');
        map.insert(Point { x: 1, y: 2 }, '2');
        map.insert(Point { x: 2, y: 2 }, '3');
        map.insert(Point { x: 0, y: 1 }, '4');
        map.insert(Point { x: 1, y: 1 }, '5');
        map.insert(Point { x: 2, y: 1 }, '6');
        map.insert(Point { x: 0, y: 0 }, '7');
        map.insert(Point { x: 1, y: 0 }, '8');
        map.insert(Point { x: 2, y: 0 }, '9');
        map.insert(Point { x: 2, y: 3 }, 'A');
        map
    }
}
