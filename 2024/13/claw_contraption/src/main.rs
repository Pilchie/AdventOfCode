use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let machines = Machine::parse_list(&contents);

    let mut total_tokens = 0;
    let mut count = 0;
    for m in machines {
        if let Some(tokens) = m.minimum_tokens() {
            total_tokens += tokens;
            count += 1;
        }
    }

    println!(
        "It would take {} total tokens to win all prizes, from {} games.",
        total_tokens, count
    );
}

struct Button {
    x: i64,
    y: i64,
    cost: i64,
}

impl Button {
    fn parse(line: &str, cost: i64) -> Self {
        let (_, a) = line.split_once(": ").unwrap();
        let (xstr, ystr) = a.split_once(", ").unwrap();
        let x = xstr[2..].parse::<i64>().unwrap();
        let y = ystr[2..].parse::<i64>().unwrap();

        Self { x, y, cost }
    }
}

struct Prize {
    x: i64,
    y: i64,
}

impl Prize {
    fn parse(line: &str) -> Self {
        let (_, rest) = line.split_once(": ").unwrap();
        let (xstr, ystr) = rest.split_once(", ").unwrap();
        let x = /*10000000000000 +*/ xstr[2..].parse::<i64>().unwrap();
        let y = /*10000000000000 +*/ ystr[2..].parse::<i64>().unwrap();

        Self { x, y }
    }
}

struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl Machine {
    fn parse_list(input: &str) -> Vec<Self> {
        let mut res = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let button_a = Button::parse(lines[i], 3);
            let button_b = Button::parse(lines[i + 1], 1);
            let prize = Prize::parse(lines[i + 2]);
            res.push(Self {
                button_a,
                button_b,
                prize,
            });
            i += 4;
        }
        res
    }

    fn minimum_tokens(&self) -> Option<i64> {
        if self.button_a.y * self.button_b.x == self.button_a.x * self.button_b.y {
            println!(
                "No solution (div0) for machine with prize at ({},{})",
                self.prize.x, self.prize.y
            );
            return None;
        }

        let a = (self.prize.y * self.button_b.x - self.prize.x * self.button_b.y)
            / (self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y);

        let b = (self.prize.x - a * self.button_a.x) / self.button_b.x;

        if a < 0 || b < 0 {
            // println!(
            //     "No solution (neg)  for machine with prize at ({},{}) (a/b) are: ({},{})",
            //     self.prize.x, self.prize.y, a, b
            // );
            return None;
        }

        if a >= 100 || b >= 100 {
            // println!(
            //     "No solution (too far) for machine with prize at ({},{}) (a/b) are: ({},{})",
            //     self.prize.x, self.prize.y, a, b
            // );
            return None;
        }

        let tokens = a * self.button_a.cost + b * self.button_b.cost;
        println!(
            "Solution for machine with prize at ({},{}) costs {}, (a/b) is ({}/{})",
            self.prize.x, self.prize.y, tokens, a, b
        );
        Some(tokens)
    }
}
