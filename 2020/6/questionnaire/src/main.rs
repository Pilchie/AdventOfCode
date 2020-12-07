use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};


fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    let mut vals = Vec::new();
    let mut sum = 0;

    for l in reader.lines() {
        let line = l?;
        if line.is_empty() {
            sum += Group::new(&vals).num_all_yes();
            vals.clear();
        } else {
            vals.push(line);
        }
    }

    // Collect the final group
    sum += Group::new(&vals).num_all_yes();

    println!("The sum is {}", sum);

    Ok(())
}

pub struct Group {
    vals: Vec<String>,
}

impl Group {
    pub fn new(vals: &[String]) -> Group {
        Group {
            vals: vals.to_vec(),
        }
    }

    pub fn num_any_yes(&self) -> usize {
        let mut all_yes = HashSet::new();
        for g in &self.vals {
            for c in g.chars() {
                all_yes.insert(c);
            }
        }
        all_yes.len()
    }

    pub fn num_all_yes(&self) -> usize {
        let letters = "abcdefghijklmnopqrstuvwxyz";
        let mut remaining_set : HashSet<_> = letters.chars().collect();

        for v in &self.vals {
            let h : HashSet<_> = v.chars().collect();
            let intersection = remaining_set.intersection(&h);
            remaining_set = intersection.cloned().collect();
        }

        remaining_set.len()
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::Group;

    #[test]
    fn first_example() {
        let group = Group::new(&["abcx".into(), "abcy".into(), "abcz".into()]);
        assert_eq!(6, group.num_any_yes());
    }

    #[test]
    fn second() {
        let group = Group::new(&["abc".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn third() {
        let group = Group::new(&["a".into(), "b".into(), "c".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn fourth() {
        let group = Group::new(&["ab".into(), "ac".into()]);
        assert_eq!(3, group.num_any_yes());
    }

    #[test]
    fn fifth() {
        let group = Group::new(&["a".into(), "a".into(), "a".into(), "a".into()]);
        assert_eq!(1, group.num_any_yes());
    }

    #[test]
    fn sixth() {
        let group = Group::new(&["b".into()]);
        assert_eq!(1, group.num_any_yes());
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::Group;

    #[test]
    fn second() {
        let group = Group::new(&["abc".into()]);
        assert_eq!(3, group.num_all_yes());
    }

    #[test]
    fn third() {
        let group = Group::new(&["a".into(), "b".into(), "c".into()]);
        assert_eq!(0, group.num_all_yes());
    }

    #[test]
    fn fourth() {
        let group = Group::new(&["ab".into(), "ac".into()]);
        assert_eq!(1, group.num_all_yes());
    }

    #[test]
    fn fifth() {
        let group = Group::new(&["a".into(), "a".into(), "a".into(), "a".into()]);
        assert_eq!(1, group.num_all_yes());
    }

    #[test]
    fn sixth() {
        let group = Group::new(&["b".into()]);
        assert_eq!(1, group.num_all_yes());
    }
}
