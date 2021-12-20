use std::collections::HashMap;
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

    let mut map = HashMap::new();
    map.insert(')', ('(', 3));
    map.insert(']', ('[', 57));
    map.insert('}', ('{', 1197));
    map.insert('>', ('<', 25137));

    let mut cmap = HashMap::new();
    cmap.insert('(', 1);
    cmap.insert('[', 2);
    cmap.insert('{', 3);
    cmap.insert('<', 4);

    let mut line_scores = Vec::new();
    for l in reader.lines() {
        let line = l?;

        let mut invalid = false;
        let mut stack = Vec::new();
        for c in line.chars() {
            let s = match c {
                '(' => push(&mut stack, c),
                '[' => push(&mut stack, c),
                '<' => push(&mut stack, c),
                '{' => push(&mut stack, c),
                ')' => expect(&mut stack, c, &map),
                ']' => expect(&mut stack, c, &map),
                '>' => expect(&mut stack, c, &map),
                '}' => expect(&mut stack, c, &map),
                _ => panic!("Unexpected char {}", c),
            };

            if s > 0 {
                invalid = true;
                break;
            }
        }

        let mut score = 0u128;
        if !invalid {
            while stack.len() > 0 {
                let s = stack.pop().unwrap();
                score = score * 5 + cmap.get(&s).unwrap();
            }
            line_scores.push(score);
        }
    }

    line_scores.sort();
    let score = line_scores[line_scores.len() / 2];

    println!("The syntax score was {}", score);

    Ok(())
}

fn push<T>(stack: &mut Vec<T>, c: T) -> u64 {
    stack.push(c);
    0
}

fn expect(stack: &mut Vec<char>, c: char, map: &HashMap<char, (char, u64)>) -> u64 {
    let a = stack.pop().unwrap();

    if let Some((e, s)) = map.get(&c) {
        if a != *e {
            return *s;
        }

        0
    } else {
        panic!("Unexpected");
    }
}
