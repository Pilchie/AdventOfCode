use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let conditions = ConditionSet::parse(&input);
    let tickets = Ticket::parse_nearby(&input);
    if let Some(mine) = Ticket::parse_yours(&input) {
        if let Some(res) = conditions.order_fields(&tickets) {
            let mut product = 0;
            for (name, _) in conditions.fields {
                if name.starts_with("departure") {
                    product *= mine.field_values[res[&name]];
                }
            }

            println!("The product is: {}", product);
        } else {
            println!("Didn't find a solution")
        }
    } else {
        println!("Couldn't parse my ticket")
    }

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

    fn in_ranges(v: i32, r1: &Range, r2: &Range) -> bool {
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

    pub fn invalid_field_values(&self, ticket: &Ticket) -> Vec<i32> {
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

    pub fn order_fields<'a>(&self, tickets: &[Ticket]) -> Option<HashMap<String, usize>> {
        let valid_tickets: Vec<_> = tickets.iter().filter(|t| self.is_possibly_valid(t)).collect();

        let mut valid_positions_by_name = HashMap::new();
        for (name, (r1, r2)) in self.fields.iter() {
            for i in 0..self.fields.len() {
                if valid_tickets.iter().all(|t| Range::in_ranges(t.field_values[i], r1, r2)) {
                    valid_positions_by_name.entry(name).or_insert(Vec::new()).push(i);
                }
            }
        }

        let mut p = 1;
        for (name, positions) in &valid_positions_by_name {
            p *= positions.len();
            println!("Possible for {} ({}): - {:?}", name, positions.len(), positions);
        }
        println!("Expecting up to {} combos", p);

        let mut possible_indices = Vec::new();
        let mut names = Vec::new();
        for (name, positions) in &valid_positions_by_name {
            names.push(*name);
            possible_indices.push(positions.clone());
        }

        let possible_indices = Self::generate_possible_indices(&possible_indices);
        println!("Trying {} possible combinations", possible_indices.len());
        for indices in possible_indices {
            if self.validate_indices(&names, &indices, &valid_tickets) {
                let mut res = HashMap::new();
                for i in 0..names.len() {
                    res.insert(names[i].clone(), indices[i]);
                }
                return Some(res)
            }
        }

        None
    }

    fn validate_indices<'a> (&self,
        names: &Vec<&String>,
        indices: &[usize],
        valid_tickets: &Vec<&Ticket>) -> bool
    {
        print!("Validating ");
        for i in 0..names.len() {
            if let Some((r1, r2)) = self.fields.get(names[i]) {
                let x = indices[i];
                println!("{}:{} in ({}-{},{}-{}) ", names[i], x, r1.min, r1.max, r2.min, r2.max);
                for t in valid_tickets {
                    print!("{} ", t.field_values[x]);
                    if !Range::in_ranges(t.field_values[x], r1, r2) {
                        println!("FALSE");
                        return false;
                    }
                }
                println!("");
            } else {
                panic!("Should been an entry for {}", names[i]);
            }
        }

        println!("TRUE");
        true
    }

    fn _order_example<'a>(
        &self,
        valid_positions_by_name: &'a HashMap<&'a String, Vec<usize>>,
        valid_tickets: &Vec<&Ticket>) -> Option<HashMap<String, usize>>
    {
        let ck = &String::from("class");
        let rk = &String::from("row");
        let sk = &String::from("seat");
        for cp in &valid_positions_by_name[ck] {
            for rp in &valid_positions_by_name[rk] {
                for sp in &valid_positions_by_name[sk] {
                    let (rc1, rc2) = &self.fields[ck];
                    let (rr1, rr2) = &self.fields[rk];
                    let (rs1, rs2) = &self.fields[sk];

                    if valid_tickets.iter().all(
                        |t| Range::in_ranges(t.field_values[*cp], &rc1, &rc2)
                        && Range::in_ranges(t.field_values[*rp], &rr1, &rr2)
                        && Range::in_ranges(t.field_values[*sp], &rs1, &rs2))
                        {
                            let mut res = HashMap::new();
                            res.insert(ck.clone(), *cp);
                            res.insert(rk.clone(), *rp);
                            res.insert(sk.clone(), *sp);
                            return Some(res);
                        }
                }
            }
        }

        None
    }

    fn generate_possible_indices<T: Copy+std::fmt::Debug+PartialEq>(x: &[Vec<T>]) -> Vec<Vec<T>> {
        if x.len() == 1 {
            let mut res = Vec::new();
            for item in &x[0] {
                res.push(std::iter::once(*item).collect());
            }

            return res;
        }

        let mut appended: Vec<Vec<T>> = Vec::new();
        for s_el in &x[0] {
            for next_s in Self::generate_possible_indices(&x[1..]) {
                let mut a = Vec::new();
                a.push(*s_el);
                let mut push = true;
                for n in next_s {
                    if a.contains(&n) {
                        push = false;
                        break;
                    } else {
                        a.push(n);
                    }
                }

                if push {
                    appended.push(a);
                }
            }
        }
        appended
    }
}

pub struct Ticket {
    field_values: Vec<i32>,
}

impl Ticket {
    pub fn parse_nearby(input: &str) -> Vec<Self> {
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

    pub fn parse_yours(input: &str) -> Option<Self> {
        let mut start = false;
        for line in input.lines() {
            if start {
                return Some(Ticket {
                    field_values: line.split(',').map(|s| s.parse::<i32>().unwrap()).collect(),
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

        assert_eq!(71, tickets.iter().flat_map(|t| conditions.invalid_field_values(t)).sum());
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let input = "class: 0-1 or 4-19
seat: 0-13 or 16-19
row: 0-5 or 8-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let conditions = ConditionSet::parse(input);
        let tickets = Ticket::parse_nearby(input);

        if let Some(res) = conditions.order_fields(&tickets) {
            assert_eq!(0, res["row"]);
            assert_eq!(1, res["class"]);
            assert_eq!(2, res["seat"]);
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_generated_indices() {
        let mut sets = Vec::new();
        sets.push([1, 2].to_vec());
        sets.push([2, 4].to_vec());
        sets.push([4, 6].to_vec());

        let res = ConditionSet::generate_possible_indices(&sets);
        for x in res {
            println!("{:?}", x);
        }
        assert!(false);
    }
}
