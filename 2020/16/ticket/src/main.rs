use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let conditions = ConditionSet::parse(&input);
    let tickets = Ticket::parse_nearby(&input);
    if let Some(mine) = Ticket::parse_yours(&input) {
        let res = conditions.order_fields(&tickets);
        let mut product = 1;
        for (name, _) in &conditions.fields {
            if name.starts_with("departure") {
                product *= mine.field_values[res[&name]];
            }
        }

        println!("The product is: {}", product);
    } else {
        println!("Couldn't parse my ticket")
    }

    Ok(())
}

#[derive(Debug)]
pub struct Range {
    min: usize,
    max: usize,
}

impl Range {
    pub fn new(min: &str, max: &str) -> Self {
        Range {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }

    fn in_ranges(v: usize, r1: &Range, r2: &Range) -> bool {
        v >= r1.min && v <= r1.max || v >= r2.min && v <= r2.max
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

    pub fn invalid_field_values(&self, ticket: &Ticket) -> Vec<usize> {
        let mut res = Vec::new();

        for v in &ticket.field_values {
            let vv = *v;
            let mut any_valid = false;
            for (_, (r1, r2)) in &self.fields {
                if vv >= r1.min && vv <= r1.max || vv >= r2.min && vv <= r2.max {
                    any_valid = true;
                    break;
                }
            }

            if !any_valid {
                res.push(vv);
            }
        }

        res
    }

    pub fn is_possibly_valid(&self, ticket: &Ticket) -> bool {
        for v in &ticket.field_values {
            let mut any_valid = false;
            for (_, (r1, r2)) in &self.fields {
                if Range::in_ranges(*v, r1, r2) {
                    any_valid = true;
                    break;
                }
            }

            if !any_valid {
                return false;
            }
        }

        true
    }

    pub fn order_fields<'a>(&self, tickets: &[Ticket]) -> HashMap<&String, usize> {
        let valid_tickets: Vec<_> = tickets.iter().filter(|t| self.is_possibly_valid(t)).collect();

        let mut valid_positions_by_name = HashMap::new();
        for (name, (r1, r2)) in self.fields.iter() {
            for i in 0..self.fields.len() {
                if valid_tickets.iter().all(|t| Range::in_ranges(t.field_values[i], r1, r2)) {
                    valid_positions_by_name.entry(name).or_insert(Vec::new()).push(i);
                }
            }
        }

        let mut res = HashMap::new();
        while !valid_positions_by_name.is_empty() {
            valid_positions_by_name = Self::fix_single_position(&mut res, & valid_positions_by_name);
        }

       res
    }

    fn fix_single_position<'a>(res: &mut HashMap<&'a String, usize>, valid_positions_by_name: &HashMap<&'a String, Vec<usize>>) ->  HashMap<&'a String, Vec<usize>>{
        let mut x = HashMap::new();

        for (name, positions) in valid_positions_by_name {
            if positions.len() == 1 {
                res.insert(name, positions[0]);

                for (n, p2) in valid_positions_by_name {
                    if n != name {
                        let mut p = p2.clone();
                        vec_remove_item(&mut p, &positions[0]);
                        x.insert(*n, p);
                    }
                }
                break;
            }
        }

        x
    }
}

pub struct Ticket {
    field_values: Vec<usize>,
}

impl Ticket {
    pub fn parse_nearby(input: &str) -> Vec<Self> {
        let mut res = Vec::new();
        let mut start = false;
        for line in input.lines() {
            if start {
                res.push(Ticket {
                    field_values: line.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
                });
            }

            if line == "nearby tickets:" {
                start = true;
            }
        }
        res
    }

    pub fn parse_yours(input: &str) -> Option<Self> {
        let mut start = false;
        for line in input.lines() {
            if start {
                return Some(Ticket {
                    field_values: line.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
                });
            }

            if line == "your ticket:" {
                start = true;
            }
        }

        None
    }
}

fn split_once<'a>(in_string: &'a str, split_on: &str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, split_on);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn vec_remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) -> Option<T> {
    let pos = match vec.iter().position(|x| *x == *item) {
        Some(x) => x,
        None => return None,
    };
    Some(vec.remove(pos))
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
        let tickets = Ticket::parse_nearby(input);

        assert_eq!(71, tickets.iter().flat_map(|t| conditions.invalid_field_values(t)).fold(0, |acc, n| acc + n));
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let conditions = ConditionSet::parse(input);
        let tickets = Ticket::parse_nearby(input);

        let res = conditions.order_fields(&tickets);
        assert_eq!(0, res[&String::from("row")]);
        assert_eq!(1, res[&String::from("class")]);
        assert_eq!(2, res[&String::from("seat")]);
    }
}
