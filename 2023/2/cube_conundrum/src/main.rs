use std::{cmp::max, io::BufRead};

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    reds: u32,
    blues: u32,
    greens: u32,
}

impl Game {
    fn parse(line: &str) -> Option<Self> {
        if let Some((game, rest)) = line.split_once(':') {
            if let Some((_, idstr)) = game.split_once(' ') {
                let roundstrings = rest.split(';');
                let mut rounds = Vec::new();
                for r in roundstrings {
                    let cubes = r.split(',');
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for c in cubes {
                        // NOTE: There is a leading space in 'c'
                        if let Some((numstr, color)) = c[1..].split_once(' ') {
                            let num = numstr.parse().unwrap();
                            match color {
                                "red" => red = num,
                                "green" => green = num,
                                "blue" => blue = num,
                                &_ => unimplemented!(),
                            }
                        }
                    }
                    rounds.push(Round {
                        reds: red,
                        greens: green,
                        blues: blue,
                    })
                }

                return Some(Game {
                    id: idstr.parse().unwrap(),
                    rounds: rounds,
                });
            }
        }
        None
    }

    fn is_possible(self: &Self) -> bool {
        for round in &self.rounds {
            if round.reds > 12 || round.greens > 13 || round.blues > 14 {
                return false;
            }
        }
        true
    }

    fn minimum_power(self: &Self) -> u32 {
        let mut max_r = 1;
        let mut max_g = 1;
        let mut max_b = 1;

        for r in &self.rounds {
            max_r = max(max_r, r.reds);
            max_g = max(max_g, r.greens);
            max_b = max(max_b, r.blues);
        }

        max_r * max_g * max_b
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let reader = std::io::BufReader::new(std::fs::File::open(&args[1])?);

    let mut games = Vec::new();
    for l in reader.lines() {
        let line = l?;
        if let Some(g) = Game::parse(&line) {
            games.push(g)
        }
    }

    let result_part1 = games
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .reduce(|acc, e| acc + e);

    println!("The sum of the ids is {}", result_part1.unwrap());

    let result_part2: u32 = games.iter().map(|g| g.minimum_power()).sum();
    println!("The sum of the minimum powers is {}", result_part2);
    Ok(())
}
