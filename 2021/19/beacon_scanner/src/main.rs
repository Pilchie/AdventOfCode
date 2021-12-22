use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let scanners = Scanner::parse_list(&input);

    let mut unfixed: HashSet<usize> = HashSet::new();
    for i in 1..scanners.len() {
        unfixed.insert(i);
    }

    let mut fixed = Vec::new();
    let mut offsets = Vec::new();

    fixed.reserve_exact(scanners.len());
    offsets.reserve_exact(scanners.len());
    fixed.push(scanners[0].clone());
    offsets.push(Coord{ x: 0, y: 0, z: 0});

    let mut prev_count = 0;
    while prev_count != unfixed.len() {
        prev_count = unfixed.len();
        let mut to_remove = Vec::new();
        for s in &unfixed {
            let candidate = &scanners[*s];
            let mut now_fixed = Vec::new();
            for f in &fixed {
                if let Some((done, offset)) = candidate.try_fix(f) {
                    now_fixed.push(done);
                    offsets.push(offset);
                    to_remove.push(*s);
                    break;
                }
            }

            fixed.extend(now_fixed);
        }
        for s in &to_remove {
            unfixed.remove(s);
        }
    }

    if !unfixed.is_empty() {
        print!("Failed to fix all scanners, {} remaining [", unfixed.len());
        for i in unfixed {
            print!("{}, ", i);
        }
        println!("]");

    } else {
        let mut beacons = HashSet::new();
        for f in &fixed {
            for p in &f.points {
                beacons.insert(p);
            }
        }

        println!("There are a total of {} beacons", beacons.len());
        
        let mut max = 0;
        for i in 0..offsets.len() {
            for j in 0..offsets.len() {
                let dist = offsets[i].manhattan_distance_from(&offsets[j]);
                if dist > max {
                    max = dist;
                }
            }
        }

        println!("The largest manhattan distance between two scanners is {}", max);
    }

    Ok(())
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Scanner {
    id: usize,
    points: Vec<Coord>,
}

impl Scanner {
    fn parse_list(input: &str) -> Vec<Scanner> {
        let mut lines = input.lines();

        let mut res = Vec::new();
        let mut done = false;
        while !done {
            let idline = lines.next().unwrap();
            let id = idline.split(" ").nth(2).unwrap().parse().unwrap();
            let mut points = Vec::new();
            loop {
                if let Some(line) = lines.next() {
                    if line.is_empty() {
                        res.push(Scanner {
                            id,
                            points: points.clone(),
                        });
                        break;
                    } else {
                        points.push(Coord::parse(line));
                    }
                } else {
                    res.push(Scanner {
                        id,
                        points: points.clone(),
                    });
                    done = true;
                    break;
                }
            }
        }

        res
    }

    fn try_fix(&self, fixed: &Self) -> Option<(Self, Coord)> {
        for rotation in self.rotations() {
            for sp in &rotation.points {
                for fp in &fixed.points {
                    // assume sp and fp are actually the same.  Find scanner offset, and test remaining points for 12 that align.
                    let offset = *fp - *sp;
                    let count = rotation.test(&fixed, &offset);
                    if count >= 12 {
                        println!("Fixed scanner {}, with offset {:?}", rotation.id, offset);
                        return Some((Self {
                            id: rotation.id,
                            points: rotation.points.iter().map(|p| *p + offset).collect(),
                        }, offset));
                    }
                }
            }
        }
        None
    }

    fn test(&self, fixed: &Self, offset: &Coord) -> usize {
        let mut count = 0;
        for sp in &self.points {
            for fp in &fixed.points {
                if *sp + *offset == *fp {
                    count += 1
                }
            }
        }

        count
    }

    fn rotations(&self) -> Vec<Scanner> {
        let mut res = Vec::new();

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.x,
                y: p.y,
                z: p.z,
            }).collect(),
        });

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.x,
                y: p.z,
                z: p.y,
            }).collect(),
        });

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.y,
                y: p.x,
                z: p.z,
            }).collect(),
        });

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.y,
                y: p.z,
                z: p.x,
            }).collect(),
        });

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.z,
                y: p.x,
                z: p.y,
            }).collect(),
        });

        res.push(Self {
            id: self.id,
            points: self.points.iter().map(|p| Coord {
                x: p.z,
                y: p.y,
                z: p.x,
            }).collect(),
        });

        Self::negations(&res)
    }

    fn negations(scanners: &Vec<Scanner>) -> Vec<Scanner> {
        let mut res = Vec::new();
        res.reserve_exact(8*scanners.len());
        res.extend(scanners.iter().map(|s| s.clone()));

        for s in scanners {
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: c.x,
                    y: c.y,
                    z: -c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: c.x,
                    y: -c.y,
                    z: c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: c.x,
                    y: -c.y,
                    z: -c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: -c.x,
                    y: c.y,
                    z: c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: -c.x,
                    y: c.y,
                    z: -c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: -c.x,
                    y: -c.y,
                    z: c.z,
                }).collect(),
            });
            res.push(Scanner {
                id: s.id,
                points: s.points.iter().map(|c| Coord {
                    x: -c.x,
                    y: -c.y,
                    z: -c.z,
                }).collect(),
            });
        }

        res
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn parse(input: &str) -> Self {
        Self {
            x: input.split(",").nth(0).unwrap().parse().unwrap(),
            y: input.split(",").nth(1).unwrap().parse().unwrap(),
            z: input.split(",").nth(2).unwrap().parse().unwrap(),
        }
    }

    fn manhattan_distance_from(&self, rhs: &Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs() + (self.z - rhs.z).abs()
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
