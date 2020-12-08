use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
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

    let r = rules.can_eventually_contain("shiny gold");
    println!("There are {} ways to contain shiny gold", r.len());
}

pub struct Rules<'a> {
    rules: HashMap<&'a str, Rule<'a>>,
}

impl<'a> Rules<'a> {
    pub fn parse(input: &'a str) -> Rules {
        Rules {
            rules: input
                .split('\n')
                .map(|line| {
                    let rule = Rule::parse(&line);
                    (rule.name, rule)
                })
                .collect(),
        }
    }

    pub fn can_eventually_contain(&self, bag_name: &str) -> Vec<&'a Rule> {
        let r = &self.rules[bag_name];
        let v: Vec<&Rule> = [r].into();

        self.can_contain_recursive(&v)
    }

    fn can_contain_recursive(&self, rules: &[&'a Rule]) -> Vec<&'a Rule> {
        println!("Looking for rules that can contain {:?}", rules);
        let mut new = Vec::new();
        for (n, r1) in &self.rules {
            for r2 in rules {
                if n != &r2.name {
                    if r1.can_contain_directly(r2.name) {
                        new.push(r1);
                    }
                }
            }
        }

        if new.is_empty() {
            return rules.into();
        }

        for r in rules {
            new.push(r);
        }
        self.can_contain_recursive(&new)
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
        assert_eq!(4, rules.can_eventually_contain("shiny gold").len());
    }
}
