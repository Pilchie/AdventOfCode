use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut lines = input.lines();

    let map: Vec<_> = lines
        .next()
        .unwrap()
        .bytes()
        .map(|c| match c {
            b'.' => false,
            b'#' => true,
            _ => panic!("Unexpected '{}' in map", c),
        })
        .collect();

    lines.next();

    let image = Image::parse(&mut lines);

    let step1 = image.enhance(&map);
    let step2 = step1.enhance(&map);

    println!("There are {} lit pixels", step2.pixels.len());

    Ok(())
}

struct Image {
    pixels: HashSet<Point>,
    default: bool,
    min: Point,
    max: Point,
}

impl Image {
    fn parse(lines: &mut std::str::Lines) -> Self {
        let mut pixels = HashSet::new();
        let mut y = 0;
        loop {
            if let Some(line) = lines.next() {
                for (x, c) in line.bytes().enumerate() {
                    if c == b'#' {
                        pixels.insert(Point { x: x as i32, y });
                    }
                }
            } else {
                break;
            }
            y += 1;
        }

        let (min, max) = Image::find_bounds(&pixels);
        Self {
            pixels,
            default: false,
            min,
            max,
        }
    }

    fn enhance(&self, map: &Vec<bool>) -> Self {
        let mut new = HashSet::new();
        // One row of pixels outside the current image can change.  The rest will be the default
        for y in self.min.y - 1..self.max.y + 2 {
            for x in self.min.x - 1..self.max.x + 2 {
                let lookup = self.construct_lookup(x, y);
                let val = map[lookup];
                if val {
                    new.insert(Point { x, y });
                }
            }
        }

        let default = if self.default {
            map[map.len() - 1]
        } else {
            map[0]
        };

        Self {
            pixels: new,
            default,
            min: Point {
                x: self.min.x - 1,
                y: self.min.y - 1,
            },
            max: Point {
                x: self.max.x + 1,
                y: self.max.y + 1,
            },
        }
    }

    fn construct_lookup(&self, x: i32, y: i32) -> usize {
        let mut val = 0;
        for yi in y-1..y+2 {
            for xi in x-1..x+2 {
                val = (val << 1) + if self.is_lit(xi, yi) { 1 } else { 0 };
            }
        }

        val
    }

    fn is_lit(&self, x: i32, y: i32) -> bool {
        if x < self.min.x || y < self.min.y {
            return self.default;
        }

        if x > self.max.x || y > self.max.y {
            return self.default;
        }

        self.pixels.contains(&Point { x, y })
    }

    fn find_bounds(pixels: &HashSet<Point>) -> (Point, Point) {
        let mut maxx = i32::MIN;
        let mut maxy = i32::MIN;
        let mut minx = i32::MAX;
        let mut miny = i32::MAX;

        for p in pixels {
            if p.x > maxx {
                maxx = p.x;
            }

            if p.y > maxy {
                maxy = p.y;
            }

            if p.x < minx {
                minx = p.x;
            }

            if p.y < miny {
                miny = p.y;
            }
        }

        (Point { x: minx, y: miny }, Point { x: maxx, y: maxy })
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
