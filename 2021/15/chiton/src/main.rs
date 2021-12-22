use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut risks = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, risk) in line.bytes().enumerate() {
            risks.insert(Point { x, y }, (risk - b'0') as i32);
        }
    }

    let max_x = max_x(&risks);
    let max_y = max_y(&risks);

    let path = a_star(Point {x: 0, y: 0}, Point {x: max_x, y: max_y}, &risks, max_x, max_y);

    let mut total = 0;
    for p in path.iter().skip(1) {
        print!("({},{}):{} -> ", p.x, p.y, risks[p]);
        total += risks[p];
    }
    println!();
    println!("The total risk is {}", total);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn max_x(map: &HashMap<Point, i32>) -> usize {
    let mut max = 0;
    for p in map.keys() {
        if p.x > max {
            max = p.x;
        }
    }

    max
}

fn max_y(map: &HashMap<Point, i32>) -> usize {
    let mut max = 0;
    for p in map.keys() {
        if p.y > max {
            max = p.y;
        }
    }

    max
}

fn a_star(start: Point, goal: Point, map: &HashMap<Point, i32>, max_x: usize, max_y: usize) -> Vec<Point> {
    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, heuristic(start));

    while !open_set.is_empty() {
        let current = find_min(&open_set, &f_score);

        if current == goal {
            return construct_path(&came_from, current);
        }

        open_set.remove(&current);

        // for each neighbor of current
        for neighbor in adjacent(current, max_x, max_y) {
            let tentative_g_score = match g_score.get(&current) {
                Some(s) => s + map[&neighbor],
                None => i32::MAX,
            };

            if tentative_g_score < *g_score.get(&neighbor).or(Some(&i32::MAX)).unwrap() {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score as f64 + heuristic(neighbor));

                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        } 
    }

    Vec::new()
}
 
fn heuristic(p: Point) -> f64 {
    (((p.x * p.x) + (p.y * p.y)) as f64).sqrt()
}

fn find_min(set: &HashSet<Point>, f_score: &HashMap<Point, f64>) -> Point {
    let mut min = f64::MAX;
    let mut minp = Point{x:0, y:0};
    for p in set {
        let v = *f_score.get(p).or(Some(&f64::MAX)).unwrap();
        if v < min
        {
            min = v;
            minp = *p;
        }
    }

    minp
}

fn construct_path(came_from: &HashMap<Point, Point>, point: Point) -> Vec<Point> {
    let mut res = Vec::new();
    res.reserve_exact(came_from.len());
    res.push(point);
    
    let mut current = point;

    while came_from.contains_key(&current) {
        current = came_from[&current];
        res.push(current);
    }

    res.reverse();

    res
}

fn adjacent(p: Point, max_x: usize, max_y: usize) -> Vec<Point> {
    let mut res = Vec::new();

    // left
    if p.x > 0 {
        res.push(Point { x: p.x - 1, y: p.y });
    }

    // above
    if p.y > 0 {
        res.push(Point { x: p.x, y: p.y - 1});
    }

    // right
    if p.x < max_x {
        res.push(Point { x: p.x + 1, y: p.y });
    }

    // below
    if p.y < max_y {
        res.push(Point { x: p.x, y: p.y + 1 });
    }

    res
}