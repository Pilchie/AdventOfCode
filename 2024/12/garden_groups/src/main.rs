use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let garden = Garden::parse_map(&contents);
    let cost: usize = garden.cost();
    let discounted_cost = garden.discounted_cost();
    println!("The total cost is {}, and the discounted cost is {}", cost, discounted_cost);
}

struct Region {
    plots: Vec<Point>,
    value: char,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }
}

struct Garden {
    regions: Vec<Region>,
    map: Vec<Vec<char>>,
}

impl Garden {
    fn parse_map(input: &str) -> Self {
        let mut map = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(ch);
            }
            map.push(row);
        }
        let mut regions = Vec::new();
        let mut known = HashSet::new();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let p = Point { x, y };
                if !known.contains(&p) {
                    let val = map[y][x];
                    known.insert(p);
                    let r = Self::explore_from(&map, val, &[p], &mut known);
                    regions.push(Region {
                        plots: r,
                        value: val,
                    });
                }
            }
        }

        Garden { regions, map }
    }

    fn explore_from(
        map: &[Vec<char>],
        val: char,
        points: &[Point],
        known: &mut HashSet<Point>,
    ) -> Vec<Point> {
        let mut res = points.to_vec();
        for initial in points {
            for p in Self::neighbors(map, *initial) {
                if !known.contains(&p) && map[p.y][p.x] == val {
                    res.push(p);
                    known.insert(p);
                }
            }
        }

        if res.len() != points.len() {
            return Self::explore_from(map, val, &res, known);
        }

        res
    }

    fn neighbors(map: &[Vec<char>], p: Point) -> Vec<Point> {
        let mut res = Vec::new();

        if p.y > 0 {
            res.push(Point { x: p.x, y: p.y - 1 });
        }
        if p.x > 0 {
            res.push(Point { x: p.x - 1, y: p.y });
        }
        if p.y < map.len() - 1 {
            res.push(Point { x: p.x, y: p.y + 1 });
        }
        if p.x < map[p.y].len() - 1 {
            res.push(Point { x: p.x + 1, y: p.y });
        }
        res
    }

    fn perimeter(&self, region: &Region) -> usize {
        let mut perimeter = 0;
        for p in &region.plots {
            if p.y == 0 || region.value != self.map[p.y - 1][p.x] {
                perimeter += 1;
            }
            if p.x == 0 || region.value != self.map[p.y][p.x - 1] {
                perimeter += 1;
            }
            if p.y == self.map.len() - 1 || region.value != self.map[p.y + 1][p.x] {
                perimeter += 1;
            }
            if p.x == self.map[p.y].len() - 1 || region.value != self.map[p.y][p.x + 1] {
                perimeter += 1;
            }
        }
        perimeter
    }

    fn sides(&self, region: &Region) -> usize {
        let mut side_count = 0;
        let mut top_sides = Vec::new();
        let mut left_sides = Vec::new();
        let mut bottom_sides = Vec::new();
        let mut right_sides = Vec::new();
        for p in &region.plots {
            if p.y == 0 || region.value != self.map[p.y - 1][p.x] {
                top_sides.push(*p);
            }
            if p.x == 0 || region.value != self.map[p.y][p.x - 1] {
                left_sides.push(*p);
            }
            if p.y == self.map.len() - 1 || region.value != self.map[p.y + 1][p.x] {
                bottom_sides.push(*p);
            }
            if p.x == self.map[p.y].len() - 1 || region.value != self.map[p.y][p.x + 1] {
                right_sides.push(*p);
            }
        }
        side_count += Self::horizontal(&mut top_sides);
        side_count += Self::horizontal(&mut bottom_sides);
        side_count += Self::vertical(&mut left_sides);
        side_count += Self::vertical(&mut right_sides);
        side_count
    }

    fn horizontal(points: &mut Vec<Point>) -> usize {
        points.sort_by(|p1, p2| p1.y.cmp(&p2.y).then(p1.x.cmp(&p2.x)));
        let mut lastx = None;
        let mut lasty = None;
        let mut count = 0;
        for t in points {
            if Some(t.y) != lasty || Some(t.x - 1) != lastx  {
                count += 1;
            }
            lastx = Some(t.x);
            lasty = Some(t.y);
        }
        count
    }

    fn vertical(points: &mut Vec<Point>) -> usize {
        points.sort_by(|p1, p2| p1.x.cmp(&p2.x).then(p1.y.cmp(&p2.y)));
        let mut lastx = None;
        let mut lasty = None;
        let mut count = 0;
        for t in points {
            if Some(t.x) != lastx || Some(t.y - 1) != lasty  {
                count += 1;
            }
            lastx = Some(t.x);
            lasty = Some(t.y);
        }
        count
    }

    fn cost(&self) -> usize {
        let mut cost = 0;
        for r in &self.regions {
            cost += r.area() * self.perimeter(r)
        }
        cost
    }

    fn discounted_cost(&self) -> usize {
        let mut cost = 0;
        for r in &self.regions {
            cost += r.area() * self.sides(r)
        }
        cost
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
