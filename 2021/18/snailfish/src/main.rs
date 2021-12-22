fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut value = parse_pair(&mut 0, input.lines().nth(0).unwrap());
    for line in input.lines().skip(1) {
        value = add(&value, &parse_pair(&mut 0, line));
    }

    value.print();
    println!();
    Ok(())
}

fn parse_pair(i: &mut usize, input: &str) -> Pair {
    expect(i, input, "[");
    let lhs = parse_term(i, input);
    expect(i, input, ",");
    let rhs = parse_term(i, input);
    expect(i, input, "]");

    Pair { lhs, rhs, }
}

fn parse_term(i: &mut usize, input: &str) -> Term {
    if peek(i, input, "[") {
        let pair = parse_pair(i, input);
        Term::Pair(Box::new(pair))
    } else {
        let value = parse_value(i, input);
        Term::Value(value)
    }
}

fn parse_value(i: &mut usize, input: &str) -> i32 {
    let start = *i;
    while &input[*i..*i+1] != "," && &input[*i..*i+1] != "]" {
        *i += 1;
    }

    let sub = &input[start..*i];
    sub.parse().unwrap()
}

fn expect(i: &mut usize, input: &str, expected: &str) {
    let actual = &input[*i..*i+1];
    if actual != expected {
        panic!("Expected {}, got {}", expected, actual);
    }
    *i += 1;
}

fn peek(i: &mut usize, input: &str, expected: &str) -> bool {
    let actual = &input[*i..*i+1];
    actual == expected
}

fn add(lhs: &Pair, rhs: &Pair) -> Pair {
    let mut res = Pair {
        lhs: Term::Pair(Box::new(lhs.clone())),
        rhs: Term::Pair(Box::new(rhs.clone())),
    };

    res = reduce(&res);
    res
}

fn reduce(pair: &Pair) -> Pair {
    let pairs = 1;
    let (term, _) = reduce_rec(pair, pairs);
    if let Term::Pair(p) = term {
        *p
    } else {
        panic!("Unknown")
    }
}

fn reduce_rec(pair: &Pair, pairs: u32) -> (Term, bool) {
    if pairs == 4 {
        return (Term::Pair(Box::new(explode(pair))), true)
    }

    let (lhs, stop) = match &pair.lhs {
        Term::Pair(p) => reduce_rec(&p, pairs + 1),
        Term::Value(v) => split(*v),
    };
    if stop {
        return (Term::Pair(Box::new(Pair {lhs, rhs: pair.rhs.clone()})), stop);
    }

    let (rhs, stop) = match &pair.rhs {
        Term::Pair(p) => reduce_rec(&p, pairs + 1),
        Term::Value(v) => split(*v),
    };

    (Term::Pair(Box::new(Pair{lhs, rhs})), stop)
}

fn split(v: i32) -> (Term, bool) {
    if v >= 10 {
        (Term::Pair(Box::new(Pair { lhs: Term::Value(v / 2), rhs: Term::Value(v + 1 / 2) })), true)
    } else {
        (Term::Value(v), false)
    }
}

fn explode(pair: &Pair) -> Pair {
    pair.clone()
}

enum Term {
    Value(i32),
    Pair(Box<Pair>)
}

impl Term {
    fn print(&self) {
        match self {
            Term::Value(v) => print!("{}", v),
            Term::Pair(p) => p.print(),
        }
    }
}

impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Term::Value(x) => Term::Value(*x),
            Term::Pair(p) => Term::Pair(p.clone()),
        }
    }
}

#[derive(Clone)]
struct Pair {
    lhs: Term,
    rhs: Term,
}

impl Pair {
    fn print(&self) {
        print!("[");
        self.lhs.print();
        print!(",");
        self.rhs.print();
        print!("]");
    }
}