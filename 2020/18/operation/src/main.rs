use std::io::{BufRead, BufReader};


fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    let reader = BufReader::new(std::fs::File::open(&args[1])?);
    let mut sum = 0;
    for line in reader.lines() {
        sum += evaluate(&line?)?
    }

    println!("The sum is {}", sum);

    Ok(())
}

pub fn evaluate(input: &str) -> Result<isize, Error> {
    let (res, _) = evaluate_helper(input)?;
    Ok(res)
}

fn evaluate_helper(input: &str) ->  Result<(isize, &str), Error> {
    let (mut left, mut remaining) = parse_term(&input)?;

    while remaining.len() > 0 {
        if &remaining[0..1] == ")" {
            return Ok((left, &remaining[1..]));
        }

        let (operator, r1) = parse_operator(&remaining)?;
        let (right, r2) = parse_term(&r1)?;
        remaining = r2;
        left = operator.apply(left, right);
    }

    Ok((left, remaining))
}

pub fn parse_term(input: &str) -> Result<(isize, &str), Error> {
    println!("parse_term, input: {}", input);

    match &input[0..1] {
        "0" => Ok((0, &input[1..])),
        "1" => Ok((1, &input[1..])),
        "2" => Ok((2, &input[1..])),
        "3" => Ok((3, &input[1..])),
        "4" => Ok((4, &input[1..])),
        "5" => Ok((5, &input[1..])),
        "6" => Ok((6, &input[1..])),
        "7" => Ok((7, &input[1..])),
        "8" => Ok((8, &input[1..])),
        "9" => Ok((9, &input[1..])),
        "(" => {
            let (res, rem) = evaluate_helper(&input[1..])?;
            Ok((res, rem))
        }

        _ => Err(Error::InvalidTerm),
    }

}

pub fn parse_operator(input: &str) -> Result<(Operator, &str), Error> {
    println!("parse_operator, input: {}", input);

    match &input[0..3] {
        " + " => Ok((Operator::Addition, &input[3..])),
        " * " => Ok((Operator::Multiplication, &input[3..])),
        _ => Err(Error::InvalidOperator {}),
    }
}

pub enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    pub fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Operator::Addition => left + right,
            Operator::Multiplication => left * right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidOperator,
    InvalidTerm,
    IO,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Ok(71), evaluate("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn example2() {
        assert_eq!(Ok(51), evaluate("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn example3() {
        assert_eq!(Ok(26), evaluate("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn example4() {
        assert_eq!(Ok(437), evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn example5() {
        assert_eq!(Ok(12240), evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    }

    
    #[test]
    fn example6() {
        assert_eq!(Ok(13632), evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}