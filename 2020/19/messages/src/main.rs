use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    let mut reader1 = BufReader::new(std::fs::File::open(&args[1])?);
    let rules = Rules::parse(&mut reader1)?;

    let reader2 = BufReader::new(std::fs::File::open(&args[1])?);
    let mut start = false;
    let mut count = 0;
    for l in reader2.lines() {
        let line = l?;
        if line.is_empty() {
            start = true;
            continue;
        }

        if !start {
            continue;
        }

        if rules.matches(&line) {
            count += 1;
        }
    }

    println!("There are {} rules that match", count);

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Self::IO(ioe)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(pie: std::num::ParseIntError) -> Self {
        Self::ParseInt(pie)
    }
}

#[derive(Debug)]
pub enum Token {
    RefList(Vec<usize>),
    Literal(u8),
    Or(Vec<usize>, Vec<usize>),
}

pub struct Rules {
    rules: HashMap<usize, Token>,
}

impl Rules {
    pub fn parse<T: BufRead>(reader: &mut T) -> Result<Self, Error> {
        let mut map = HashMap::new();
        for l in reader.lines() {
            let line = l?;
            if line.is_empty() {
                break;
            }

            println!("parsing {}", line);

            let (ridstr, rest) = split_once(&line, ":");
            let rid = ridstr.parse::<usize>()?;
            let tokens: Vec<_> = rest[1..].split(' ').collect();
            if tokens.len() == 1 && tokens[0].starts_with('\"') {
                let t = Token::Literal(tokens[0].bytes().nth(1).unwrap());
                map.insert(rid, t);
            } else {
                let mut v1 = Vec::new();
                let mut v2 = Vec::new();
                let mut or = false;
                for i in 0..tokens.len() {
                    if or {
                        v2.push(tokens[i].parse::<usize>().unwrap());
                    } else if tokens[i] == "|" {
                        or = true;
                    } else {
                        v1.push(tokens[i].parse::<usize>().unwrap());
                    }
                }

                if or {
                    map.insert(rid, Token::Or(v1, v2));
                } else {
                    map.insert(rid, Token::RefList(v1));
                }
            }
        }

        Ok(Self{
            rules: map,
        })
    }

    pub fn matches(&self, candidate: &str) -> bool {
        let base_rule = &self.rules[&0];
        let (res, rem) = self.matches_rule(candidate, base_rule);
        res && rem.len() == 0
    }

    pub fn matches_rule<'a>(&self, candidate: &'a str, rule: &Token) -> (bool, &'a str) {
        println!("Matching {:?} against {}", rule, candidate);
        match rule {
            Token::Literal(v) => if candidate.bytes().nth(0).unwrap() == *v {
                println!("- succeeded");
                (true, &candidate[1..])
            } else {
                println!("- failed1");
                (false, candidate)
            }
            Token::Or(left, right) => {
                let (res1, rem1) = self.matches_list(candidate, left);
                if res1 {
                    (res1, rem1)
                } else {
                    self.matches_list(candidate, right)
                }
            }
            Token::RefList(vec) => self.matches_list(candidate, vec),
        }
    }

    fn matches_list<'a>(&self, candidate: &'a str, list: &[usize]) -> (bool, &'a str) {
        let mut remaining = candidate;
        for v in list {
            let (res, rem) = self.matches_rule(remaining, &self.rules[v]);
            if res {
                remaining = rem;
            } else {
                println!("- failed3");
                return (false, remaining);
            }
        }
        println!("- succeeded");
        (true, remaining)
    }
}

fn split_once<'a>(in_string: &'a str, split_on: &str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, split_on);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

#[cfg(test)]
mod tests_part1 {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let rules = Rules::parse(&mut Cursor::new("0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\""))?;
        assert!(rules.matches("aab"));
        assert!(rules.matches("aba"));

        Ok(())
    }

    #[test]
    fn example() -> Result<(), Error> {
        let rules = Rules::parse(&mut Cursor::new("0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\""))?;

        assert!(rules.matches("ababbb"));
        assert!(!rules.matches("bababa"));
        assert!(rules.matches("abbbab"));
        assert!(!rules.matches("aaabbb"));
        assert!(!rules.matches("aaaabbb"));

        Ok(())
    }
}