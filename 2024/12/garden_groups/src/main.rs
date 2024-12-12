use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let regions = Region::parse_map(&contents);
    let cost: usize = regions.iter().map(|r| r.cost()).sum();
    println!("The total cost is {}", cost);
}

struct Region {
    plots: Vec<Point>
}

impl Region {
    fn parse_map(_input: &str) -> Vec<Self> {
        Vec::new()
    }

    fn cost(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
