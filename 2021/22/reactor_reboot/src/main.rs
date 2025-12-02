use core::ops::RangeInclusive;
use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::parse(line));
    }

    let mut reactor = Reactor::new();
    for instr in &instructions {
        reactor = reactor.apply(instr);
    }

    println!("There are {} cubes on", reactor.cubes.len());

    Ok(())
}

struct Reactor {
    cubes: HashSet<Point>,
}

impl Reactor {
    fn new() -> Self {
        Self { cubes: HashSet::new(), }
    }

    fn apply(&self, instr: &Instruction) -> Self {
        let mut new = self.cubes.clone();

        for x in instr.xrange.clone().filter(|v| *v >= -50 && *v <= 50) {
            for y in instr.yrange.clone().filter(|v| *v >= -50 && *v <= 50) {
                for z in instr.zrange.clone().filter(|v| *v >= -50 && *v <= 50) {
                    let p = Point { x, y, z };
                    if instr.target {
                        new.insert(p);
                    } else {
                        new.remove(&p);
                    }
                }
            }
        }

        Self { cubes: new, }
    }
}

struct Instruction {
    target: bool,
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
    zrange: RangeInclusive<i32>,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let (target, rangesstr) = input.split_once(" ").unwrap();
        let ranges: Vec<_> = rangesstr.split(",").map(|r| {
            let (s, e) = &r[2..].split_once("..").unwrap();
            RangeInclusive::new(
                s.parse::<i32>().unwrap(),
                e.parse::<i32>().unwrap(),
            )
        }).collect();

        Self {
            target: match target { "on" => true, "off" => false, _ => panic!("Unexpected target") },
            xrange: ranges[0].clone(),
            yrange: ranges[1].clone(),
            zrange: ranges[2].clone(),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}