use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let machines = Machine::parse_list(&contents);

    let mut total_tokens = 0.0;
    let mut count = 0;
    let mut winnable = Vec::new();
    for (i, m) in machines.iter().enumerate() {
        if let Some(tokens) = m.minimum_tokens() {
            total_tokens += tokens;
            count += 1;
            winnable.push(i);
        }
    }

    println!(
        "It would take {} total tokens to win all prizes, from {} games.",
        total_tokens, count
    );

    println!("winnable machines: {:?}", winnable);
}

struct Button {
    x: f64,
    y: f64,
    cost: f64,
}

impl Button {
    fn parse(line: &str, cost: f64) -> Self {
        let (_, a) = line.split_once(": ").unwrap();
        let (xstr, ystr) = a.split_once(", ").unwrap();
        let x = xstr[2..].parse::<f64>().unwrap();
        let y = ystr[2..].parse::<f64>().unwrap();

        Self { x, y, cost }
    }
}

struct Prize {
    x: f64,
    y: f64,
}

impl Prize {
    fn parse(line: &str) -> Self {
        let (_, rest) = line.split_once(": ").unwrap();
        let (xstr, ystr) = rest.split_once(", ").unwrap();
        let x = 10000000000000.0 + xstr[2..].parse::<f64>().unwrap();
        let y = 10000000000000.0 + ystr[2..].parse::<f64>().unwrap();

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
            let button_a = Button::parse(lines[i], 3.0);
            let button_b = Button::parse(lines[i + 1], 1.0);
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

    fn minimum_tokens(&self) -> Option<f64> {
        let a = (self.prize.y * self.button_b.x - self.prize.x * self.button_b.y)
            / (self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y);

        let b = (self.prize.x - a * self.button_a.x) / self.button_b.x;

        if a.fract() > f64::EPSILON || b.fract() > f64::EPSILON {
            return None
        }

        let tokens = a * self.button_a.cost + b * self.button_b.cost;
        Some(tokens)
    }
}
