use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    let mut reader1 = BufReader::new(std::fs::File::open(&args[1])?);
    let mut rules = Rules::parse(&mut reader1)?;
    rules.apply_changes();

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
        let results = self.matches_rule(&[candidate], base_rule);
        println!("Finished with results: {:?}", results);
        results.iter().any(|r| r.len() == 0)
    }

    pub fn matches_rule<'a>(&self, candidates: &[&'a str], rule: &Token) -> Vec<&'a str> {
        let mut remainders = Vec::new();
        for candidate in candidates {
            println!("Matching {:?} against {}", rule, candidate);
            match rule {
                Token::Literal(l) => match candidate.bytes().nth(0) {
                    Some(v) => if v == *l {
                        println!("- succeeded");
                        remainders.push(&candidate[1..]);
                    } else {
                        println!("- failed1");
                    },
                    None => {},
                }
                Token::Or(left, right) => {
                    let mut rem1 = self.matches_list(candidates, left);
                    remainders.append(&mut rem1);

                    let mut rem2 = self.matches_list(candidates, right);
                    remainders.append(&mut rem2);
                }
                Token::RefList(vec) => {
                    let mut rem = self.matches_list(candidates, vec);
                    remainders.append(&mut rem);
                }
            }
        }
        remainders
    }

    fn matches_list<'a>(&self, candidates: &[&'a str], list: &[usize]) -> Vec<&'a str> {
        if list.is_empty() {
            return candidates.to_vec();
        }

        let mut remaining = Vec::new();
        for candidate in candidates {
            let first = self.matches_rule(&[candidate], &self.rules[&list[0]]);
            let mut rest = self.matches_list(&first, &list[1..]);
            remaining.append(&mut rest);
        }
        remaining
    }

    pub fn apply_changes(&mut self) {
        self.rules.insert(8, Token::Or([42].to_vec(), [42, 8].to_vec()));
        self.rules.insert(11, Token::Or([42, 31].to_vec(), [42, 11, 31].to_vec()));
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

#[cfg(test)]
mod tests_part2 {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_before() -> Result<(), Error> {
        let rules = Rules::parse(&mut Cursor::new("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"))?;

        assert!(!rules.matches("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"));
        assert!(rules.matches("bbabbbbaabaabba"));
        assert!(!rules.matches("babbbbaabbbbbabbbbbbaabaaabaaa"));
        assert!(!rules.matches("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
        assert!(!rules.matches("bbbbbbbaaaabbbbaaabbabaaa"));
        assert!(!rules.matches("bbbababbbbaaaaaaaabbababaaababaabab"));
        assert!(rules.matches("ababaaaaaabaaab"));
        assert!(rules.matches("ababaaaaabbbaba"));
        assert!(!rules.matches("baabbaaaabbaaaababbaababb"));
        assert!(!rules.matches("abbbbabbbbaaaababbbbbbaaaababb"));
        assert!(!rules.matches("aaaaabbaabaaaaababaa"));
        assert!(!rules.matches("aaaabbaaaabbaaa"));
        assert!(!rules.matches("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
        assert!(!rules.matches("babaaabbbaaabaababbaabababaaab"));
        assert!(!rules.matches("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));

        Ok(())
    }

    #[test]
    fn example_after() -> Result<(), Error> {
        let mut rules = Rules::parse(&mut Cursor::new("42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"))?;

        rules.apply_changes();

        assert!(!rules.matches("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"));
        assert!(rules.matches("bbabbbbaabaabba"));
        assert!(rules.matches("babbbbaabbbbbabbbbbbaabaaabaaa"));
        assert!(rules.matches("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"));
        assert!(rules.matches("bbbbbbbaaaabbbbaaabbabaaa"));
        assert!(rules.matches("bbbababbbbaaaaaaaabbababaaababaabab"));
        assert!(rules.matches("ababaaaaaabaaab"));
        assert!(rules.matches("ababaaaaabbbaba"));
        assert!(rules.matches("baabbaaaabbaaaababbaababb"));
        assert!(rules.matches("abbbbabbbbaaaababbbbbbaaaababb"));
        assert!(rules.matches("aaaaabbaabaaaaababaa"));
        assert!(!rules.matches("aaaabbaaaabbaaa"));
        assert!(rules.matches("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"));
        assert!(!rules.matches("babaaabbbaaabaababbaabababaaab"));
        assert!(rules.matches("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));

        Ok(())
    }
}