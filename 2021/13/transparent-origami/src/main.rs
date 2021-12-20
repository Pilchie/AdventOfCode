use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut paper = Paper::parse(&input);

    let mut at_folds = false;
    for line in input.lines() {
        if !at_folds && line.len() != 0 {
            continue;
        }

        if line.len() == 0 {
            at_folds = true;
            continue;
        }

        let f = &line["fold along ".len()..];
        paper = match f.split_once("=") {
            Some((axis, value)) => {
                let v = value.parse::<usize>().unwrap();
                match axis {
                    "x" => paper.fold_x(v),
                    "y" => paper.fold_y(v),
                    _ => panic!("Unknown axis: '{}'", axis),
                }
            }
            None => {
                panic!("Unexpected line: '{}'", line);
            }
        };
    }

    paper.print();

    println!("Paper has {} dots", paper.points.len());

    Ok(())
}

struct Paper {
    points: HashSet<Point>,
}

impl Paper {
    fn parse(input: &str) -> Self {
        let mut points = HashSet::new();
        for line in input.lines() {
            if line.len() == 0 {
                break;
            }

            if let Some((x, y)) = line.split_once(",") {
                points.insert(Point {
                    x: x.parse::<usize>().unwrap(),
                    y: y.parse::<usize>().unwrap(),
                });
            } else {
                panic!("Unexpected input");
            }
        }

        Self { points }
    }

    fn fold_x(self: &Self, val: usize) -> Self {
        let mut new = HashSet::new();

        for x in 0..val {
            for y in 0..self.max_y() + 1 {
                let p = Point { x, y };
                if self.points.contains(&p) {
                    new.insert(p);
                }
            }
        }

        for x in val + 1..self.max_x() + 1 {
            for y in 0..self.max_y() + 1 {
                if self.points.contains(&Point { x, y }) {
                    let diff_from_edge = x - val;
                    let new_x = val - diff_from_edge;
                    new.insert(Point { x: new_x, y: y });
                }
            }
        }

        Self { points: new }
    }

    fn fold_y(self: &Self, val: usize) -> Self {
        let mut new = HashSet::new();

        for x in 0..self.max_x() + 1 {
            for y in 0..val {
                let p = Point { x, y };
                if self.points.contains(&p) {
                    new.insert(p);
                }
            }

            for y in val + 1..self.max_y() + 1 {
                if self.points.contains(&Point { x, y }) {
                    let diff_from_edge = y - val;
                    let new_y = val - diff_from_edge;
                    new.insert(Point { x: x, y: new_y });
                }
            }
        }

        Self { points: new }
    }

    fn max_x(self: &Self) -> usize {
        let mut max = 0;
        for p in &self.points {
            if p.x > max {
                max = p.x
            }
        }

        max
    }

    fn max_y(self: &Self) -> usize {
        let mut max = 0;
        for p in &self.points {
            if p.y > max {
                max = p.y
            }
        }

        max
    }

    fn print(self: &Self) {
        for y in 0..self.max_y() + 1 {
            for x in 0..self.max_x() + 1 {
                if self.points.contains(&Point { x, y }) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
