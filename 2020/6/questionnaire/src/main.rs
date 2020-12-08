use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};


fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let reader = io::BufReader::new(File::open(&args[1])?);
    let sum = Group::parse(reader)?.iter().fold(0, |acc, x| acc + x.num_all_yes());

    println!("The sum is {}", sum);

    Ok(())
}

pub struct Group {
    vals: Vec<String>,
}

impl Group {
    pub fn parse<T: BufRead>(reader: T) -> Result<Vec<Group>, std::io::Error> {
        let mut groups = Vec::new();
        let mut vals = Vec::new();
        for l in reader.lines() {
            let line = l?;
            if line.is_empty() {
                groups.push(Group::new(vals.clone()));
                vals.clear();
            } else {
                vals.push(line);
            }
        }
    
        // Collect the final group
        groups.push(Group::new(vals));

        Ok(groups)
    }

    pub fn new<I: Into<Vec<String>>>(vals: I) -> Group {
        Group {
            vals: vals.into(),
        }
    }

    pub fn num_any_yes(&self) -> usize {
        let all_yes : HashSet<_> = self.vals.iter().flat_map(|c| c.chars()).collect();
        all_yes.len()
    }

    pub fn num_all_yes(&self) -> usize {
        let letters = "abcdefghijklmnopqrstuvwxyz";
        let mut remaining_set : HashSet<_> = letters.chars().collect();

        for v in &self.vals {
            let h : HashSet<_> = v.chars().collect();
            remaining_set.retain(|c| h.contains(c));
        }

        remaining_set.len()
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::Group;

    #[test]
    fn first_example() {
        let group = Group::new(["abcx".into(), "abcy".into(), "abcz".into()]);
        assert_eq!(6, group.num_any_yes());
    }

    #[test]
    fn second() {
        let group = Group::new(["abc".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn third() {
        let group = Group::new(["a".into(), "b".into(), "c".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn fourth() {
        let group = Group::new(["ab".into(), "ac".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn fifth() {
        let group = Group::new(["a".into(), "a".into(), "a".into(), "a".into()]);
        assert_eq!(1, group.num_any_yes());
    }

    #[test]
    fn sixth() {
        let group = Group::new(["b".into()]);
        assert_eq!(1, group.num_any_yes());
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::Group;

    #[test]
    fn second() {
        let group = Group::new(["abc".into()]);
        assert_eq!(3, group.num_all_yes());
    }

    #[test]
    fn third() {
        let group = Group::new(["a".into(), "b".into(), "c".into()]);
        assert_eq!(0, group.num_all_yes());
    }

    #[test]
    fn fourth() {
        let group = Group::new(["ab".into(), "ac".into()]);
        assert_eq!(1, group.num_all_yes());
    }

    #[test]
    fn fifth() {
        let group = Group::new(["a".into(), "a".into(), "a".into(), "a".into()]);
        assert_eq!(1, group.num_all_yes());
    }

    #[test]
    fn sixth() {
        let group = Group::new(["b".into()]);
        assert_eq!(1, group.num_all_yes());
    }
}
