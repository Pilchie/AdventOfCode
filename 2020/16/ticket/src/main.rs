use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let conditions = ConditionSet::parse(&input);
    let tickets = Ticket::parse(&input);

    println!("Ticket scanning error rate: {}", tickets.iter().flat_map(|t| conditions.invalid_field_values(t)).sum::<i32>());

    Ok(())
}

#[derive(Debug)]
pub struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn new(min: &str, max: &str) -> Self {
        Range {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }
}

pub struct ConditionSet {
    fields: HashMap<String, (Range, Range)>,
}

impl ConditionSet {
    pub fn parse(input: &str) -> Self {
        let mut fields = HashMap::new();
        for line in input.lines() {
            if line == "" { 
                break;
            }
            let (field, ranges) = split_once(line, ": ");
            let (first, second) = split_once(ranges, " or ");
            let (min1, max1) = split_once(first, "-");
            let (min2, max2) = split_once(second, "-");
            fields.insert(
                String::from(field),
                (Range::new(min1, max1), Range::new(min2, max2)),
            );
        }

        ConditionSet { fields: fields }
    }

    pub fn invalid_field_values(&self, ticket: &Ticket) -> Vec<i32> {
        let mut res = Vec::new();

        for v in &ticket.field_values {
            let vv = *v;
            let mut any_valid = false;
            for (_, (r1, r2)) in &self.fields {
                print!("Checking {} against {:?} and {:?}: ", vv, r1, r2);
                if vv >= r1.min && vv <= r1.max || vv >= r2.min && vv <= r2.max {
                    println!("valid");
                    any_valid = true;
                    break;
                } else {
                    println!("***INVALID***");
                }
            }

            if !any_valid {
                res.push(vv);
            }
        }

        res
    }
}

pub struct Ticket {
    field_values: Vec<i32>,
}

impl Ticket {
    pub fn parse(input: &str) -> Vec<Self> {
        let mut res = Vec::new();
        let mut start = false;
        for line in input.lines() {
            if start {
                res.push(Ticket {
                    field_values: line.split(',').map(|s| s.parse::<i32>().unwrap()).collect(),
                });
            }

            if line == "nearby tickets:" {
                start = true;
            }
        }
        res
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
    use super::*;

    #[test]
    fn test() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let conditions = ConditionSet::parse(input);
        let tickets = Ticket::parse(input);

        assert_eq!(71, tickets.iter().flat_map(|t| conditions.invalid_field_values(t)).sum());
    }
}
