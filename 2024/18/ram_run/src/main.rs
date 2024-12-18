use std::{collections::{HashMap, HashSet, VecDeque}, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let test = path.contains("test");

    let (maxx, maxy, _bytes) = match test {
        true => (6, 6, 12),
        false => (70, 70, 1024),
    };

    let start = Point { x: 0, y: 0 };
    let end = Point { x: maxx, y: maxy };

    let mut corrupted = HashSet::new();
    for line in contents.lines() {
        let (xstr, ystr) = line.split_once(",").unwrap();
        let x = xstr.parse::<u8>().unwrap();
        let y = ystr.parse::<u8>().unwrap();
        corrupted.insert(Point { x, y });
        //println!("Trying with {} ({}) bytes have fallen", line, corrupted.len());

        if steps_to_exit(start, end, maxx, maxy, &corrupted) == None {
            println!("No exit once {} fell", line);
            break;
        }
    }
}

fn steps_to_exit(start: Point, end: Point, maxx: u8, maxy: u8, corrupted: &HashSet<Point>) -> Option<u32> {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    let mut min = u32::MAX;
    queue.push_back(State { point: start, steps: 0, });
    seen.insert(start, 0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if current.point == end {
            if current.steps < min {
                min = current.steps;
            }
        }

        if current.steps > min {
            continue;
        }

        for next in next_points(&current.point, maxx, maxy, &corrupted) {
            if let Some(existing) = seen.get(&next) {
                if *existing > current.steps + 1 {
                    seen.insert(next, current.steps + 1);
                    queue.push_back(State { point: next, steps: current.steps + 1, });
                } else {
                    // We've already found a cheaper or equal way to get here. Skip it.
                }
            }
            else {
                seen.insert(next, current.steps + 1);
                queue.push_back(State { point: next, steps: current.steps + 1, });
            }
        }
    }

    match min == u32::MAX {
        true => None,
        false => Some(min)
    }
}

fn next_points(p: &Point, maxx: u8, maxy: u8, corrupted: &HashSet<Point>) -> Vec<Point>
{
    let mut res = Vec::new();
    if p.x > 0 {
        add_if_not_corrupted(Point { x: p.x - 1, y: p.y, }, &mut res, corrupted);
    }

    if p.y > 0 {
        add_if_not_corrupted(Point { x: p.x, y: p.y - 1, }, &mut res, corrupted);
    }

    if p.x < maxx {
        add_if_not_corrupted(Point { x: p.x + 1, y: p.y, }, &mut res, corrupted);
    }

    if p.y < maxy {
        add_if_not_corrupted(Point { x: p.x, y: p.y + 1, }, &mut res, corrupted);
    }

    res
}

fn add_if_not_corrupted(p: Point, res: &mut Vec<Point>, corrupted: &HashSet<Point>) {
    if !corrupted.contains(&p) {
        res.push(p);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    point: Point,
    steps: u32,
}
