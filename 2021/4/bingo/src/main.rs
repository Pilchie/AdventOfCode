use std::io::BufRead;

#[derive(Debug)]
#[allow(dead_code)]
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

    let mut lines = Vec::new();
    for l in reader.lines() {
        lines.push(l?);
    }

    let (called, mut remaining) = parse(&lines);
    let mut winners = Vec::new();
    for n in called {
        remaining = apply(n, &remaining, &mut winners);
        if remaining.len() == 0 {
            break;
        }
    }

    let (winning_number, lastset) = &winners[winners.len() - 1];
    let last = &lastset[lastset.len() - 1];

    println!("Last Bingo score is {}", winning_number * last.score());

    Ok(())
}

fn apply(n: i32, boards: &[Board], winners: &mut Vec<(i32, Vec<Board>)>) -> Vec<Board> {
    let mut winners_this_round = Vec::new();
    let mut remaining = Vec::new();

    for b in boards {
        let mut bc = b.clone();
        bc.mark(n);

        match bc.wins() {
            true => winners_this_round.push(bc),
            false => remaining.push(bc),
        };
    }

    println!("Found {} winners for {}, {} remaining", winners_this_round.len(), n, remaining.len());

    if winners_this_round.len() > 0 {
        winners.push((n, winners_this_round));
    }

    remaining
}

fn parse(input: &[String]) -> (Vec<i32>, Vec<Board>) {
    let numbers : Vec<_> = input[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let mut boards = Vec::new();
    for i in (2..input.len()).step_by(6) {
        boards.push(Board::parse(&input[i..i+5]));
    }

    (numbers, boards)
}

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<(i32, bool)>>
}

impl Board {
    fn parse(input: &[String]) -> Board {
        let mut rows = Vec::new();
        for line in input {
            rows.push(line.split_ascii_whitespace().map(|x| (x.parse::<i32>().unwrap(), false)).collect());
        }

        Board {
            rows
        }
    }

    fn wins(self: &Board) -> bool {
        for y in 0..self.rows.len() {
            let mut row_wins = true;
            for x in 0..self.rows[y].len() {
                if !self.rows[y][x].1 {
                    row_wins = false;
                }
            }
            if row_wins {
                return true;
            }
        }

        for x in 0..self.rows[0].len() {
            let mut col_wins = true;
            for y in 0..self.rows.len() {
                if !self.rows[y][x].1 {
                    col_wins = false;
                }
            }
            if col_wins {
                return true;
            }
        }

        false
    }

    fn score(self: &Board) -> i32 {
        let mut score = 0;
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                if !self.rows[y][x].1 {
                    score += self.rows[y][x].0;
                }
            }
        }

        score
    }

    fn mark(self: &mut Board, num: i32) {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                if self.rows[y][x].0 == num {
                    self.rows[y][x].1 = true;
                }
            }
        }
    }

    fn _print(self: &Self) {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                let found = match self.rows[y][x].1 {
                    true => '*',
                    false => ' ',
                };
                print!("{}{}", self.rows[y][x].0, found);
            }
            println!();
        }
        println!();
    }
}