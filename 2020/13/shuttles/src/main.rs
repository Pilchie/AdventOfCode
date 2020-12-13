use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let args : Vec<_> = std::env::args().collect();
    let reader = std::io::BufReader::new(std::fs::File::open(&args[1])?);
    let mut lines = reader.lines();
    if let Some(time_str) = lines.next() {
        if let Some(input) = lines.next() {
            let time = time_str?.parse::<usize>().unwrap();
            let schedule = Schedule::parse(&input?);
            let (id, wait) = schedule.next_after(time);
            println!("The next bus is id: {}, in {} minutes - product {}", id, wait, id*wait);
        }
    }
    Ok(())
}

pub struct Schedule {
    ids: Vec<usize>,
}

impl Schedule {
    pub fn parse(input: &str) -> Self {
        Self {
            ids: input
                .split(',')
                .filter(|x| x != &"x")
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }

    pub fn next_after(&self, time: usize) -> (usize, usize) {
        // Note - x doesn't cancel out because of integer math
        let nexts = self.ids.iter().map(|x| (*x, (time + x)/x * x));
        let mut mx = usize::MAX;
        let mut mt = usize::MAX;
        for (x, t) in nexts {
            println!("(x, t): ({},{})", x, t);
            if t < mt {
                mx = x;
                mt = t;
            }
        }
        (mx, mt - time)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let schedule = Schedule::parse("7,13,x,x,59,x,31,19");
        let (id, wait) = schedule.next_after(939);

        assert_eq!(59, id);
        assert_eq!(5, wait);
    }
}
