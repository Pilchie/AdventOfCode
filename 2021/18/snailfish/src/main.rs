fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut max = 0;

    for (i1, line1) in input.lines().enumerate() {
        for (i2, line2) in input.lines().enumerate() {
            if i1 != i2 {
                let p1 = Pair::parse_str(line1);
                let p2 = Pair::parse_str(line2);
                let sum = Pair::add(&p1, &p2);
                let magnitude = sum.magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }

    println!();

    println!("The magnitude is: {}", max);
    Ok(())
}

enum Term {
    Value(i32),
    Pair(Box<Pair>)
}

impl Term {
    fn _print(&self) {
        match self {
            Term::Value(v) => print!("{}", v),
            Term::Pair(p) => p._print(),
        }
    }

    fn append_to(&self, vec: &mut Vec<Element>) {
        match self {
            Term::Value(v) => vec.push(Element::Value(*v)),
            Term::Pair(p) => p.append_to(vec),
        }  
    }

    fn magnitude(&self) -> i32 {
        match self {
            Term::Value(v) => *v,
            Term::Pair(p) => p.magnitude(),
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
    fn parse_str(input: &str) -> Self {
        let elements = Pair::lex(input);

        let mut i = 0;
        Pair::parse(&mut i, &elements)
    }

    fn parse(i: &mut usize, input: &Vec<Element>) -> Self {
        Pair::expect(i, input, &Element::Open);
        let lhs = Pair::parse_term(i, input);
        Pair::expect(i, input, &Element::Comma);
        let rhs = Pair::parse_term(i, input);
        Pair::expect(i, input, &Element::Close);
    
        Self { lhs, rhs, }
    }

    fn parse_term(i: &mut usize, input: &Vec<Element>) -> Term {
        if Pair::peek(i, input, &Element::Open) {
            let pair = Pair::parse(i, input);
            Term::Pair(Box::new(pair))
        } else {
            let value = Pair::parse_value(i, input);
            Term::Value(value)
        }
    }

    fn parse_value(i: &mut usize, input: &Vec<Element>) -> i32 {
        let res = match input[*i] {
            Element::Value(value) => value,
            _ => panic!("Unexpected input element, expected a value, got {:?}", input[*i]),
        };
        *i += 1;
        res
    }

    fn expect(i: &mut usize, input: &Vec<Element>, expected: &Element) {
        let actual = &input[*i];
        if actual != expected {
            panic!("Expected {:?}, got {:?}", expected, actual);
        }
        *i += 1;
    }
    
    fn peek(i: &mut usize, input: &Vec<Element>, expected: &Element) -> bool {
        let actual = &input[*i];
        actual == expected
    }

    
    fn lex(input: &str) -> Vec<Element> {
        let mut res = Vec::new();

        let mut i = 0;
        while i < input.len() {
            match &input[i..i+1] {
                "[" => { i += 1; res.push(Element::Open)},
                "," => { i += 1; res.push(Element::Comma)},
                "]" => { i += 1; res.push(Element::Close)},
                _ => res.push(Element::Value(Pair::lex_value(&mut i, input))),
            }
        }

        res
    }
    
    fn lex_value(i: &mut usize, input: &str) -> i32 {
        let start = *i;
        while &input[*i..*i+1] != "," && &input[*i..*i+1] != "]" {
            *i += 1;
        }
    
        let sub = &input[start..*i];
        sub.parse().unwrap()
    }

    fn add(lhs: &Pair, rhs: &Pair) -> Pair {
        let sum = Pair {
            lhs: Term::Pair(Box::new(lhs.clone())),
            rhs: Term::Pair(Box::new(rhs.clone())),
        };
    
        sum.reduce()
    }
    
    fn reduce(&self) -> Self {
        let mut elements = Vec::new();
        self.append_to(&mut elements);

        print!("Starting with: ");
        Pair::print_elements(&elements);
        loop {
            let (res, reduced) = Pair::try_explode(&elements);
            if reduced {
                elements = res;
                print!("Exploded to:   ");
                Pair::print_elements(&elements);
            } else {
                let (res, reduced) = Pair::try_split(&elements);
                if reduced {
                    elements = res;
                    print!("Split to:      ");
                    Pair::print_elements(&elements);                    
                } else {
                    break;
                }
            }
        }
    
        let mut i = 0;
        Pair::parse(&mut i, &elements)
    }

    fn try_explode(elements: &Vec<Element>) -> (Vec<Element>, bool) {
        let mut reduced = false;
        let mut opens = 0;
        let mut copy = Vec::new();
        let mut pending = None;
        copy.reserve(elements.len());
        let mut i = 0;
        while i < elements.len() {
            if reduced {
                if let Some(rhs) = pending {
                    if let Element::Value(value) = elements[i] {
                        pending = None;
                        copy.push(Element::Value(rhs + value));
                    } else {
                        copy.push(elements[i].clone());
                    }
                } else {
                    copy.push(elements[i].clone());
                }
                i += 1;
            } else {
                match &elements[i] {
                    Element::Open => {
                        opens += 1;
                        if opens == 5 {
                            if let Element::Value(lhs) = elements[i+1] {
                                if let Element::Value(rhs) = elements[i+3] {
                                    for x in (0..copy.len()).rev() {
                                        if let Element::Value(value) = copy[x] {
                                            copy[x] = Element::Value(value + lhs);
                                            break;
                                        }
                                    }

                                    pending = Some(rhs);
                                    copy.push(Element::Value(0));

                                    // skip past open, value, comma, value, close
                                    i += 5;
                                    reduced = true;
                                } else {
                                    panic!("Expected value, got {:?}", elements[i+3]);
                                }
                            } else {
                                panic!("Expected value, got {:?}", elements[i+1]);
                            }

                        } else {
                            copy.push(elements[i].clone());
                            i += 1;
                        }
                    },
                    Element::Close => {
                        opens -= 1;
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                    Element::Value(_) => {
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                    Element::Comma => {
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                }
            }
        }

        (copy, reduced)
    }

    fn try_split(elements: &Vec<Element>) -> (Vec<Element>, bool) {
        let mut reduced = false;
        let mut copy = Vec::new();
        copy.reserve(elements.len());
        let mut i = 0;
        while i < elements.len() {
            if reduced {
                copy.push(elements[i].clone());
                i += 1;
            } else {
                match &elements[i] {
                    Element::Value(v) => {
                        if *v < 10 {
                            copy.push(elements[i].clone());
                        } else {
                            copy.push(Element::Open);
                            copy.push(Element::Value(*v / 2));
                            copy.push(Element::Comma);
                            copy.push(Element::Value((*v + 1)/ 2));
                            copy.push(Element::Close);
                            reduced = true;
                        }
                        i += 1;
                    },
                    Element::Open => {
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                    Element::Close => {
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                    Element::Comma => {
                        copy.push(elements[i].clone());
                        i += 1;
                    },
                }
            }
        }

        (copy, reduced)
    }

    fn print_elements(elements: &Vec<Element>) {
        for e in elements {
            match e {
                Element::Open => print!("["),
                Element::Comma => print!(","),
                Element::Close => print!("]"),
                Element::Value(value) => print!("{}", value),
            }
        }
        println!();
    }

    fn append_to(&self, vec: &mut Vec<Element>) {
        vec.push(Element::Open);
        self.lhs.append_to(vec);
        vec.push(Element::Comma);
        self.rhs.append_to(vec);
        vec.push(Element::Close);
    }
    
    fn _print(&self) {
        print!("[");
        self.lhs._print();
        print!(",");
        self.rhs._print();
        print!("]");
    }

    fn magnitude(&self) -> i32 {
        3 * self.lhs.magnitude() + 2 * self.rhs.magnitude()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Open,
    Comma,
    Close,
    Value(i32),
}