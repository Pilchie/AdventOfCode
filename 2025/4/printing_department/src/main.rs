use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let mut rolls = HashSet::new();
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in contents.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert(Point {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    part_one(&rolls, rows, cols);
    part_two(rolls, rows, cols);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn adjacent_points(&self, rows: i32, cols: i32) -> Vec<Point> {
        let mut res = Vec::new();
        if self.row > 0 {
            if self.col > 0 {
                res.push(Point {
                    row: self.row - 1,
                    col: self.col - 1,
                })
            }
            res.push(Point {
                row: self.row - 1,
                col: self.col,
            });
            if self.col < cols - 1 {
                res.push(Point {
                    row: self.row - 1,
                    col: self.col + 1,
                });
            }
        }
        if self.row < rows - 1 {
            if self.col > 0 {
                res.push(Point {
                    row: self.row + 1,
                    col: self.col - 1,
                })
            }
            res.push(Point {
                row: self.row + 1,
                col: self.col,
            });
            if self.col < cols - 1 {
                res.push(Point {
                    row: self.row + 1,
                    col: self.col + 1,
                });
            }
        }
        if self.col > 0 {
            res.push(Point {
                row: self.row,
                col: self.col - 1,
            });
        }
        if self.col < cols - 1 {
            res.push(Point {
                row: self.row,
                col: self.col + 1,
            });
        }
        res
    }

    fn is_accessible(&self, rolls: &HashSet<Point>, rows: i32, cols: i32) -> bool {
        let mut adjacent = 0;
        for a in self.adjacent_points(rows, cols) {
            if rolls.contains(&a) {
                adjacent += 1;
            }
        }
        adjacent < 4
    }
}

fn part_one(rolls: &HashSet<Point>, rows: i32, cols: i32) {
    let accessible = find_accessible(rolls, rows, cols);
    println!("Part 1: There are {} rolls of paper can be accessed by a forklift", accessible.len());
}

fn part_two(rolls: HashSet<Point>, rows: i32, cols: i32) { 
    let mut remaining = rolls.clone();
    loop {
        let to_be_removed = find_accessible(&remaining, rows, cols);
        if to_be_removed.is_empty() {
            break;
        }
        for p in to_be_removed {
            remaining.remove(&p);
        }
    }

    println!("Part 2: A total of {} rolls of paper can be removed", rolls.len() - remaining.len());
}

fn find_accessible(rolls: &HashSet<Point>, rows: i32, cols: i32) -> HashSet<Point> {
    let mut accessible = HashSet::new();

    for p in rolls {
        if p.is_accessible(rolls, rows, cols) {
            accessible.insert(p.clone());
        }
    }

    accessible
}