use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let machines = Machine::parse_list(&contents);

    let mut total_tokens = 0;
    for m in machines {
        if let Some(tokens) = m.minimum_tokens() {
            total_tokens += tokens;
        }
    }

    println!(
        "It would take {} total tokens to win all prizes.",
        total_tokens
    );
}

struct Button {
    x: usize,
    y: usize,
    cost: usize,
}

impl Button {
    fn parse(line: &str, cost: usize) -> Self {
        let (_, a) = line.split_once(": ").unwrap();
        let (xstr, ystr) = a.split_once(", ").unwrap();
        let x = xstr[2..].parse::<usize>().unwrap();
        let y = ystr[2..].parse::<usize>().unwrap();

        Self { x, y, cost }
    }
}

struct Prize {
    x: usize,
    y: usize,
}

impl Prize {
    fn parse(line: &str) -> Self {
        let (_, rest) = line.split_once(": ").unwrap();
        let (xstr, ystr) = rest.split_once(", ").unwrap();
        let x = xstr[2..].parse::<usize>().unwrap();
        let y = ystr[2..].parse::<usize>().unwrap();

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

    fn minimum_tokens(&self) -> Option<usize> {
        let mut solutions = Vec::new();
        for a in 0..101 {
            for b in 0..101 {
                let xloc = a * self.button_a.x + b * self.button_b.x; 
                let yloc = a * self.button_a.y + b * self.button_b.y;
                if xloc == self.prize.x && yloc == self.prize.y {
                    let tokens = a * self.button_a.cost + b * self.button_b.cost;
                    solutions.push(tokens);
                } else if xloc > self.prize.x || yloc > self.prize.y { 
                    break;
                }
            }
        }
        solutions.sort();
        solutions.first().copied()
    }
}
