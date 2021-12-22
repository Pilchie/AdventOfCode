use std::ops::Range;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let s = &input["target area: ".len()..];
    let (xrange, yrange) = match s.split_once(", ") {
        Some((x, y)) => (parse_range(x), parse_range(y)),
        None => panic!("Unexpected input {}", s),
    };

    println!("Target is x={:?}, u={:?}", xrange, yrange);

    let mut overall_max = 0;
    for x_vel in 0..1000 {
        for y_vel in 0..1000 {
            //print!("Trying {},{} -> ", x_vel, y_vel);
            let mut probe = Probe::new(x_vel, y_vel);
            let mut max = 0;
            for _ in 1.. {
                probe = probe.step();
                if probe.y_pos > max {
                    max = probe.y_pos;
                }

                if probe.x_pos >= xrange.start
                    && probe.x_pos <= xrange.end
                    && probe.y_pos >= yrange.start
                    && probe.y_pos <= yrange.end
                {
                    if max > overall_max {
                        overall_max = max;
                    }
                    //println!("Target area reached!");
                    break;
                }
        
                if probe.y_pos < yrange.start {
                    //println!("Missed target!");
                    break;
                }
            }
       
        }
    }

    println!("Overall max value is {}", overall_max);

    Ok(())
}

fn parse_range(r: &str) -> Range<i32> {
    let s = &r[2..];
    let (min, max) = match s.split_once("..") {
        Some((minstr, maxstr)) => (
            minstr.parse::<i32>().unwrap(),
            maxstr.parse::<i32>().unwrap(),
        ),
        None => panic!("Unexpected range: {}", s),
    };

    Range {
        start: min,
        end: max,
    }
}

struct Probe {
    x_vel: i32,
    y_vel: i32,
    x_pos: i32,
    y_pos: i32,
}

impl Probe {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            x_vel,
            y_vel,
            x_pos: 0,
            y_pos: 0,
        }
    }

    fn step(self: &Self) -> Self {
        Self {
            x_pos: self.x_pos + self.x_vel,
            y_pos: self.y_pos + self.y_vel,
            x_vel: if self.x_vel == 0 {
                0
            } else if self.x_vel < 0 {
                self.x_vel + 1
            } else {
                self.x_vel - 1
            },
            y_vel: self.y_vel - 1,
        }
    }
}
