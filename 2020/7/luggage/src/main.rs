use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

fn main() -> Result<(), std::io::Error> {
    let args : Vec<_> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let rules = Rules::parse(&input);

    if let Some(r) = rules.can_eventually_contain("shiny gold") {
        println!("There are {} ways to contain shiny gold", r.len());
    } else {
        println!("Failed to find any rules that contain shiny gold");
    }

    Ok(())
}

pub struct Rules<'a> {
    rules: HashSet<Rule<'a>>,
}

impl<'a> Rules<'a> {
    pub fn parse(input: &'a str) -> Rules {
        Rules {
            rules: input
                .split('\n')
                .map(|line| Rule::parse(&line))
                .collect(),
        }
    }

    pub fn can_eventually_contain(&self, bag_name: &str) -> Option<HashSet<&'a Rule>> {
        if let Some(r) = self.rules.iter().filter(|x| x.name == bag_name).nth(0) {
            let mut v = HashSet::new();
            v.insert(r);
            v = self.can_contain_recursive(v, 0);
            v.remove(r);
            Some(v)
        } else {
            None
        }
    }

    fn can_contain_recursive(&self, rules: HashSet<&'a Rule<'a>>, depth: usize) -> HashSet<&'a Rule> {
        let names : Vec<&'a str> = rules.iter().map(|r| r.name).collect();
        println!("Looking for rules that can contain {:?}", names);
        let mut new = HashSet::new();
        for r1 in &self.rules {
            for r2 in &rules {
                if r1.can_contain_directly(r2.name) {
                    new.insert(r1);
                }
            }
        }

        let len = rules.len();
        for r in rules {
            new.insert(r);
        }

        if new.len() == len {
            return new
        }

        self.can_contain_recursive(new, depth + 1)
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    name: &'a str,
    contains: HashSet<&'a str>,
}

impl<'a> Rule<'a> {
    pub fn parse(line: &str) -> Rule {
        let mut spaces = Vec::new();
        for (pos, c) in line.chars().enumerate() {
            if c == ' ' {
                spaces.push(pos);
            }
        }
        spaces.push(line.len());

        let mut contains = Vec::new();
        if &line[spaces[3]..] == " no other bags." {
        } else {
            let mut space = 3;
            while space < spaces.len() - 1 {
                let x = &line[spaces[space + 1] + 1..spaces[space + 3]];
                contains.push(x);
                space += 4;
            }
        }

        // Couldn't get collect to work below :(
        let mut hs = HashSet::new();
        for s in contains {
            hs.insert(s);
        }

        Rule {
            name: &line[0..spaces[1]],
            contains: hs,
        }
    }

    pub fn can_contain_directly(&self, bag_name: &str) -> bool {
        self.contains.iter().any(|s| s == &bag_name)
    }
}

impl<'a> PartialEq for Rule<'a> {
    fn eq(&self, rhs : &Rule<'a>) -> bool {
        self.name == rhs.name
    }
}

impl<'a> Eq for Rule<'a> {
}

impl<'a> Hash for Rule<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rules = Rules::parse(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );

        if let Some(x) = rules.can_eventually_contain("shiny gold") {
            assert_eq!(4, x.len());
        } else {
            unreachable!();
        }
    }
}


