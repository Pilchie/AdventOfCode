use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    part_one(&contents);
    part_two(&contents);
}

#[derive(Debug, Eq, Hash, PartialEq)]
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
}

fn part_one(contents: &str) {
    let mut accessible = 0;
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

    for p in &rolls {
        let mut adjacent = 0;
        for a in p.adjacent_points(rows, cols) {
            if rolls.contains(&a) {
                adjacent += 1;
            }
        }

        if adjacent < 4 {
            accessible += 1;
        }
    }

    println!("Part 1: There are {} rolls of paper can be accessed by a forklift", accessible);
}

fn part_two(contents: &str) {    
}