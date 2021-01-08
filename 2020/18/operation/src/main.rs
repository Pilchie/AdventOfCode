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

fn evaluate_helper(input: &str) -> Result<(isize, &str), Error> {
    let (mut left, mut remaining) = parse_product(input)?;

    while !remaining.is_empty() {
        let t = peek_token(remaining)?;
        match t {
            Token::Multiplication => {
                let (right, r) = parse_product(&remaining[3..])?;
                remaining = r;
                left = left * right;
            }
            Token::CloseParen => {
                return Ok((left, &remaining[1..]));
            }
            _ => {
                return Err(Error::UnexpectedToken(t));
            }
        }
    }

    Ok((left, remaining))
}

fn parse_product(input: &str) -> Result<(isize, &str), Error> {
    let (mut left, mut remaining) = parse_term(input)?;
    loop {
        let t = peek_token(remaining)?;
        match t {
            Token::Addition => {
                let (right, rem) = parse_term(&remaining[3..])?;
                remaining = rem;
                left += right;
            }
            Token::Multiplication => return Ok((left, remaining)),
            Token::CloseParen => return Ok((left, remaining)),
            Token::EOF => return Ok((left, remaining)),
            _ => return Err(Error::UnexpectedToken(t)),
        }
    }
}

fn parse_term(input: &str) -> Result<(isize, &str), Error> {
    let t = peek_token(input)?;
    match t {
        Token::OpenParen => evaluate_helper(&input[1..]),
        Token::Number(num) => Ok((num, &input[1..])),
        _ => Err(Error::UnexpectedToken(t)),
    }
}

fn peek_token(input: &str) -> Result<Token, Error> {
    for c in input.bytes() {
        if c == b' ' {
            continue;
        }

        return match c {
            b'+' => Ok(Token::Addition),
            b'*' => Ok(Token::Multiplication),
            b'(' => Ok(Token::OpenParen),
            b')' => Ok(Token::CloseParen),
            b'0' => Ok(Token::Number(0)),
            b'1' => Ok(Token::Number(1)),
            b'2' => Ok(Token::Number(2)),
            b'3' => Ok(Token::Number(3)),
            b'4' => Ok(Token::Number(4)),
            b'5' => Ok(Token::Number(5)),
            b'6' => Ok(Token::Number(6)),
            b'7' => Ok(Token::Number(7)),
            b'8' => Ok(Token::Number(8)),
            b'9' => Ok(Token::Number(9)),
            _ => Err(Error::InvalidOperator {}),
        };
    }

    Ok(Token::EOF)
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Addition,
    Multiplication,
    OpenParen,
    CloseParen,
    Number(isize),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidOperator,
    InvalidTerm,
    IO,
    UnexpectedToken(Token),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Ok(231), evaluate("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn example2() {
        assert_eq!(Ok(51), evaluate("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn example3() {
        assert_eq!(Ok(46), evaluate("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn example4() {
        assert_eq!(Ok(1445), evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn example5() {
        assert_eq!(
            Ok(669060),
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            Ok(23340),
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
        //                              ((  6   * 9) * ( 15   *   14 ) + 6) + 2 + 4 * 2
        //                              (     54     *        210      + 6) + 2 + 4 * 2
        //                              (     54     *        216)          + 2 + 4 * 2
        //                                         11,664                   + 2 + 4 * 2
        //                                                          11,666      + 4 * 2
        //                                                                  11,670  * 2
        //                                                                       23,340
    }

    #[test]
    fn mine() {
        assert_eq!(Ok(480), evaluate("8 * 3 + 9 + 3 * 4"));
    }
}
