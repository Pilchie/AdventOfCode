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

    let mut outputs = 0;
    for l in reader.lines() {
        let line = l?;
        if let Some(entry) = Entry::parse(&line) {
            let mut smap = HashMap::new();
            let mut vmap = HashMap::new();

            for s in &entry.signal_patterns {
                if let Some(val) = match s.len() {
                    2 => Some(1),
                    3 => Some(7),
                    4 => Some(4),
                    5 => None,
                    6 => None,
                    7 => Some(8),
                    _ => panic!("Too many segments"),
                } {
                    smap.insert(s, val);
                    vmap.insert(val, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 5 && contains_all(&s, &vmap[&7]) {
                    smap.insert(s, 3);
                    vmap.insert(3, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 6 && contains_all(&s, &vmap[&3]) && contains_all(&s, &vmap[&4]) {
                    smap.insert(s, 9);
                    vmap.insert(9, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 6 && !smap.contains_key(&s) && !contains_all(&s, &vmap[&1]) {
                    smap.insert(s, 6);
                    vmap.insert(6, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 6 && !smap.contains_key(&s) {
                    smap.insert(s, 0);
                    vmap.insert(0, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 5 && !smap.contains_key(&s) && contains_all(&vmap[&6], s) {
                    smap.insert(s, 5);
                    vmap.insert(5, s);
                }
            }

            for s in &entry.signal_patterns {
                if s.len() == 5 && !smap.contains_key(&s) {
                    smap.insert(s, 2);
                    vmap.insert(2, s);
                }
            }

            let mut output = 0;
            for o in entry.output {
                let val = smap[&o];
                output = output * 10 + val;
            }

            println!("The output is {}", output);
            outputs += output;
        } else {
            panic!("Couldn't parse a line");
        }
    }

    println!("There are a total of the outputs is {}", outputs);

    Ok(())
}

struct Entry {
    signal_patterns: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl Entry {
    fn parse(input: &str) -> Option<Self> {
        if let Some((s, o)) = input.split_once(" | ") {
            return Some(Self {
                signal_patterns: s
                    .split_ascii_whitespace()
                    .map(|x| sorted_chars(x))
                    .collect(),
                output: o
                    .split_ascii_whitespace()
                    .map(|x| sorted_chars(x))
                    .collect(),
            });
        }

        None
    }
}

fn sorted_chars(s: &str) -> Vec<char> {
    let mut sorted = s.chars().collect::<Vec<char>>();
    sorted.sort();
    sorted
}

fn contains_all<T: PartialEq>(source: &[T], elements: &[T]) -> bool {
    for e in elements {
        if !source.contains(e) {
            return false;
        }
    }

    true
}
