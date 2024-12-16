use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let debug = false;
    let mut lobby = Lobby::parse(&contents, 103, 101);

    let mut count = 0;
    loop {
        lobby = lobby.tick();
        count += 1;
        if debug {
            println!("After {} seconds", count);
            lobby.draw();
        }
        if lobby.has_dense_quadrant() {
            break;
        }
    }

    println!("Final state:");
    lobby.draw();

    println!("Safety score after {} seconds is: {}", count, lobby.safety_score());
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

struct Robot {
    position: Vector,
    velocity: Vector,
}

struct Lobby {
    height: i32,
    width: i32,
    robots: Vec<Robot>,
}

impl Vector {
    fn parse(input: &str) -> Self {
        let (xs, ys) = input[2..].split_once(",").unwrap();
        Self {
            x: xs.parse::<i32>().unwrap(),
            y: ys.parse::<i32>().unwrap(),
        }
    }

    fn advance(&self, other: &Vector, height: i32, width: i32) -> Self {
        let mut x = (self.x + other.x) % width;
        if x < 0 {
            x = width + x;
        }
        let mut y = (self.y + other.y) % height;
        if y < 0 {
            y = height + y;
        }
        Self { x, y }
    }
}

impl Robot {
    fn parse(input: &str) -> Self {
        let (pstr, vstr) = input.split_once(" ").unwrap();
        Self {
            position: Vector::parse(pstr),
            velocity: Vector::parse(vstr),
        }
    }

    fn advance(&self, height: i32, width: i32) -> Self {
        Self {
            position: self.position.advance(&self.velocity, height, width),
            velocity: self.velocity,
        }
    }
}

impl Lobby {
    fn parse(input: &str, height: i32, width: i32) -> Self {
        let mut robots = Vec::new();
        for line in input.lines() {
            robots.push(Robot::parse(line));
        }
        Self {
            height,
            width,
            robots,
        }
    }

    fn tick(&self) -> Self {
        let mut new = Vec::new();
        for r in &self.robots {
            new.push(r.advance(self.height, self.width));
        }

        Self {
            height: self.height,
            width: self.width,
            robots: new,
        }
    }

    fn count_quadrants(&self) -> [i32;4] {
        let mut quadrants: [i32; 4] = [0, 0, 0, 0];
        for r in &self.robots {
            if let Some(q) = self.quadrant(r) {
                quadrants[q] += 1;
            }
        }

        quadrants
    }

    fn safety_score(&self) -> i32 {
        let quadrants = self.count_quadrants();
        let mut res = 1;
        for q in quadrants {
            res *= q;
        }
        res
    }

    fn has_dense_quadrant(&self) -> bool {
        let quadrants = self.count_quadrants();
        for i1 in 0..4 {
            let mut found_close = false;
            for i2 in 0..4 {
                let ratio = quadrants[i1] as f64 / quadrants[i2] as f64;
                if i1 != i2 && ratio > 0.5 && ratio < 2.0 {
                    found_close = true;
                }
            }

            if !found_close {
                return true;
            }
        }

        false
    }

    fn quadrant(&self, robot: &Robot) -> Option<usize> {
        let midx = self.width / 2;
        let midy = self.height / 2;
        if robot.position.x < midx {
            if robot.position.y < midy {
                return Some(0);
            } else if robot.position.y > midy {
                return Some(1);
            }
        } else if robot.position.x > midx {
            if robot.position.y < midy {
                return Some(2);
            } else if robot.position.y > midy {
                return Some(3);
            }
        }

        None
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut count = 0;
                for r in &self.robots {
                    if r.position.x == x && r.position.y == y {
                        count += 1;
                    }
                }

                if count == 0 {
                    print!(".");
                } else {
                    print!("{}", count);
                }
            }
            println!();
        }
    }
}
