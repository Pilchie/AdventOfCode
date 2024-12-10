use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let map = Map::parse(&contents);
    let mut sum_scores = 0;
    let mut sum_ratings = 0;

    for th in map.trailheads() {
        let score = map.score(th);
        let rating = map.rating(th);
        println!("The trailhead at ({:?}) has score {} and rating {}", th, score, rating);
        sum_scores += score;
        sum_ratings += rating;
    }

    println!("The sum of all the scores is {}, and the sum of the ratings is {}", sum_scores, sum_ratings);
}

struct Map {
    heights: Vec<Vec<u8>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut heights = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for ch in line.bytes() {
                row.push(ch - b'0');
            }
            heights.push(row);
        }
        Self { heights }
    }

    fn trailheads(&self) -> Vec<Point> {
        let mut res = Vec::new();
        for y in 0..self.heights.len() {
            for x in 0..self.heights[y].len() {
                if self.heights[y][x] == 0 {
                    res.push(Point { x, y });
                }
            }
        }

        res
    }

    fn score(&self, trailhead: Point) -> usize {
        let mut set = HashSet::new();
        for p in self.peaks(&vec![trailhead]) {
            set.insert(p);
        }

        set.len()
    }

    fn rating(&self, trailhead: Point) -> usize {
        let mut collector = HashSet::new();
        self.paths(&vec![trailhead], &mut collector);
        collector.len()
    }

    fn peaks(&self, path: &[Point]) -> Vec<Point> {
        let mut res = Vec::new();
        for n in self.next(path.last().unwrap()) {
            if self.heights[n.y][n.x] == 9 {
                res.push(Point { x: n.x, y: n.y });
            } else {
                let mut nextpath = path.to_vec();
                nextpath.push(n);
                let mut peaks = self.peaks(&nextpath);
                res.append(&mut peaks);
            }
        }

        res
    }

    fn paths(&self, path: &[Point], collector: &mut HashSet<Vec<Point>>) {
        for n in self.next(path.last().unwrap()) {
            let mut newpath = path.to_vec();
            newpath.push(n);

            if self.heights[n.y][n.x] == 9 {
                collector.insert(newpath);
            } else {
                self.paths(&newpath, collector);
            }
        }
    }

    fn next(&self, p: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        let pval = self.heights[p.y][p.x];
        if p.y > 0 && self.heights[p.y - 1][p.x] == pval + 1 {
            res.push(Point { x: p.x, y: p.y - 1 });
        }

        if p.x > 0 && self.heights[p.y][p.x - 1] == pval + 1 {
            res.push(Point { x: p.x - 1, y: p.y });
        }

        if p.y < self.heights.len() - 1 && self.heights[p.y + 1][p.x] == pval + 1 {
            res.push(Point { x: p.x, y: p.y + 1 });
        }

        if p.x < self.heights[p.y].len() - 1 && self.heights[p.y][p.x + 1] == pval + 1 {
            res.push(Point { x: p.x + 1, y: p.y });
        }

        res
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
