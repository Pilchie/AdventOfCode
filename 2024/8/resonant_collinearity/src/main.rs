use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let map = Map::parse(&contents);
    let antinodes = map.count_antinodes();
    println!("There are {} antipodes.", antinodes);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Map {
    antennas: HashMap<char, Vec<Point>>,
    height: i32,
    width: i32,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in input.lines().enumerate() {
            width = 0;
            for (x, ch) in line.chars().enumerate() {
                if ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' || ch >= '0' && ch <= '9' {
                    let p = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
                    if let Some(v) = antennas.get_mut(&ch) {
                        v.push(p);
                    } else {
                        antennas.insert(ch, vec![p]);
                    }
                }
                width += 1;
            }
            height += 1;
        }

        Map {
            antennas,
            height,
            width,
        }
    }

    fn count_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for set in self.antennas.values() {
            for i in 0..set.len() {
                for j in i + 1..set.len() {
                    for p in self.ans(&set[i], &set[j]) {
                        if self.contains(&p) {
                            antinodes.insert(p);
                        }
                    }
                }
            }
        }

        antinodes.len()
    }

    fn ans(&self, p1: &Point, p2: &Point) -> Vec<Point> {
        let mut ans = Vec::new();
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        if p1.y < p2.y {
            if p1.x < p2.x {
                ans.push(Point::new(p1.x - dx, p1.y - dy));
                ans.push(Point::new(p2.x + dx, p2.y + dy));
            } else {
                ans.push(Point::new(p1.x + dx, p1.y - dy));
                ans.push(Point::new(p2.x - dx, p2.y + dy));
            }
        } else {
            if p1.x < p2.x {
                ans.push(Point::new(p1.x - dx, p1.y + dy));
                ans.push(Point::new(p2.x + dx, p2.y - dy));
            } else {
                ans.push(Point::new(p1.x - dx, p1.y - dy));
                ans.push(Point::new(p2.x + dx, p2.y + dy));
            }
        }
        ans
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}