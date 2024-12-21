use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let test = path.contains("test");

    let orig_map = Map::parse(&contents);
    let orig_path = orig_map.path_faster_than(usize::MAX).unwrap();
    println!("Original time is {}", orig_path.len() - 1);

    let min_savings = match test {
        true => 1,
        false => 100,
    };

    let mut time_to_end = HashMap::new();
    for i in 0..orig_path.len() {
        time_to_end.insert(orig_path[i], orig_path.len() - i);
    }

    let mut cheats = 0;
    let mut ends = HashSet::new();
    for p in orig_path {
        for e in orig_map.cheat_ends(&p) {
            if !ends.contains(&(p, e)) {
                ends.insert((p, e));
                if let Some(tte) = time_to_end.get(&e) {
                    let ttp = time_to_end[&p];
                    if ttp > *tte + 2 {
                        let savings = ttp - *tte - 2;
                        if savings >= min_savings {
                            cheats += 1;
                        }
                    }
                }
            }
        }
    }
    println!(
        "There are {} cheats that save at least {} picoseconds.",
        cheats, min_savings
    );
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

struct Map {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Map {
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
                    '.' => {}
                    'S' => start = Point { x, y },
                    'E' => end = Point { x, y },
                    _ => unreachable!(),
                }
                x += 1;
            }
            y += 1;
        }

        Self { walls, start, end }
    }

    fn path_faster_than(&self, time: usize) -> Option<Vec<Point>> {
        let mut queue = VecDeque::new();
        let mut seen = HashMap::new();

        queue.push_back(vec![self.start.clone()]);
        seen.insert(self.start, 0);

        while !queue.is_empty() {
            let cur = queue.pop_back().unwrap();
            let last = cur.last().unwrap();
            if last == &self.end {
                if cur.len() < time {
                    return Some(cur);
                }
            } else if cur.len() > time {
                continue;
            } else {
                for n in self.neighbor_paths(last) {
                    let existing = seen.get(&n).unwrap_or(&usize::MAX);
                    if *existing <= cur.len() + 1 {
                        continue;
                    }
                    let mut next = cur.clone();
                    next.push(n);
                    queue.push_back(next);
                    seen.insert(n, cur.len() + 1);
                }
            }
        }

        None
    }

    fn neighbor_paths(&self, point: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        let maxx = self.walls.iter().map(|p| p.x).max().unwrap();
        let maxy = self.walls.iter().map(|p| p.y).max().unwrap();
        if point.x > 0
            && !self.walls.contains(&Point {
                x: point.x - 1,
                y: point.y,
            })
        {
            res.push(Point {
                x: point.x - 1,
                y: point.y,
            })
        }

        if point.x < maxx
            && !self.walls.contains(&Point {
                x: point.x + 1,
                y: point.y,
            })
        {
            res.push(Point {
                x: point.x + 1,
                y: point.y,
            })
        }

        if point.y > 0
            && !self.walls.contains(&Point {
                x: point.x,
                y: point.y - 1,
            })
        {
            res.push(Point {
                x: point.x,
                y: point.y - 1,
            })
        }

        if point.y < maxy
            && !self.walls.contains(&Point {
                x: point.x,
                y: point.y + 1,
            })
        {
            res.push(Point {
                x: point.x,
                y: point.y + 1,
            })
        }

        res
    }

    fn cheat_ends(&self, point: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        let maxx = self.walls.iter().map(|p| p.x).max().unwrap();
        let maxy = self.walls.iter().map(|p| p.y).max().unwrap();
        if point.x > 1 {
            res.push(Point {
                x: point.x - 2,
                y: point.y,
            })
        }

        if point.x < maxx - 1 {
            res.push(Point {
                x: point.x + 2,
                y: point.y,
            })
        }

        if point.y > 1 {
            res.push(Point {
                x: point.x,
                y: point.y - 2,
            })
        }

        if point.y < maxy - 1 {
            res.push(Point {
                x: point.x,
                y: point.y + 2,
            })
        }

        res
    }
}
