use std::io::BufRead;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IO(ioe)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(pie: std::num::ParseIntError) -> Self {
        Error::Parse(pie)
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    let reader = std::io::BufReader::new(std::fs::File::open(&args[1])?);
    let mut lines = reader.lines();
    let part1 = false;
    if part1 {
        if let Some(time_str) = lines.next() {
            if let Some(input) = lines.next() {
                let time = time_str?.parse::<usize>().unwrap();
                let schedule = Schedule::parse(&input?);
                let (id, wait) = schedule.next_after(time);
                println!(
                    "The next bus is id: {}, in {} minutes - product {}",
                    id,
                    wait,
                    id * wait
                );
            }
        }
    } else {
        lines.next(); // skip first line
        if let Some(input) = lines.next() {
            let schedule = SchedulePart2::parse(&input?)?;
            let time = schedule.first_time_chinese_remainder_theorem();
            println!("The first time to meet the constraint is {}", time);
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
        let nexts = self.ids.iter().map(|x| (*x, (time + x) / x * x));
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

pub struct SchedulePart2 {
    ids_and_offsets: Vec<(i128, i128)>,
}

impl SchedulePart2 {
    pub fn parse(input: &str) -> Result<Self, std::num::ParseIntError> {
        let mut res = Vec::new();
        for (i, x) in input.split(',').enumerate() {
            if x != "x" {
                res.push((x.parse::<i128>()?, i as i128));
            }
        }
        Ok(Self {
            ids_and_offsets: res,
        })
    }

    pub fn first_time(&self) -> i128 {
        let mut max_index = 0;
        for i in 0..self.ids_and_offsets.len() {
            if self.ids_and_offsets[i].0 > self.ids_and_offsets[max_index].0 {
                max_index = i;
            }
        }

        println!(
            "Trying multiples of {}, with {} items",
            self.ids_and_offsets[max_index].0,
            self.ids_and_offsets.len()
        );

        let mut cur = self.ids_and_offsets[max_index].0 - self.ids_and_offsets[max_index].1;
        loop {
            println!("cur is {}", cur);
            let mut valid = true;

            for (x, o) in &self.ids_and_offsets {
                if (cur + o) % x != 0 {
                    valid = false;
                    break;
                }
            }

            if valid {
                return cur;
            }

            cur = cur + self.ids_and_offsets[max_index].0;
        }
    }

    pub fn first_time_chinese_remainder_theorem(&self) -> i128 {
        first_time_rec(&self.ids_and_offsets)
    }
}

fn first_time_rec(ids_and_offsets: &Vec<(i128, i128)>) -> i128 {
    println!("Recursing on {:?}", ids_and_offsets);
    if ids_and_offsets.len() == 1 {
        return (ids_and_offsets[0].0 - ids_and_offsets[0].1) % ids_and_offsets[0].0;
    }

    let mut res = Vec::new();
    for i in 0..ids_and_offsets.len() / 2 {
        let (n1, a1) = ids_and_offsets[2 * i];
        let (n2, a2) = ids_and_offsets[2 * i + 1];
        res.push(reduce(n1, a1, n2, a2));
    }

    if ids_and_offsets.len() % 2 == 1 {
        res.push(ids_and_offsets[ids_and_offsets.len() - 1]);
    }

    first_time_rec(&res)
}

fn reduce(n1: i128, a1: i128, n2: i128, a2: i128) -> (i128, i128) {
    let (m1, m2) = extended_euclidean_algorithm(n1, n2);
    let nr = n1 * n2;
    let ar = (a1 * m2 * n2 + a2 * m1 * n1) % nr;

    println!(
        "  Reduced a1:{}, n1:{}, a2:{}, n2:{} => nr:{}, ar:{} with m1, m2 = {}, {}",
        a1, n1, a2, n2, nr, ar, m1, m2
    );
    (nr, ar)
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let quotient = old_r / r;

        let tr = r;
        r = old_r - quotient * r;
        old_r = tr;

        let ts = s;
        s = old_s - quotient * s;
        old_s = ts;

        let tt = t;
        t = old_t - quotient * t;
        old_t = tt;
    }

    (old_s, old_t)
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

    #[test]
    fn part2_1() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("7,13,x,x,59,x,31,19")?;
        assert_eq!(1068781, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn part2_2() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("17,x,13,19")?;
        assert_eq!(3417, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn part2_3() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("67,7,59,61")?;
        assert_eq!(754018, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn part2_4() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("67,x,7,59,61")?;
        assert_eq!(779210, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn part2_5() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("67,7,x,59,61")?;
        assert_eq!(1261476, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn part2_6() -> Result<(), std::num::ParseIntError> {
        let schedule = SchedulePart2::parse("1789,37,47,1889")?;
        assert_eq!(1202161486, schedule.first_time_chinese_remainder_theorem());
        Ok(())
    }

    #[test]
    fn test_extended_euclidean_algorithm() {
        let x = extended_euclidean_algorithm(3, 4);
        assert_eq!((-1, 1), x);
    }
}
