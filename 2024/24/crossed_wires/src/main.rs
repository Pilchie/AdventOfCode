use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut circuit = Circuit::parse(&contents);
    while !circuit.is_steady_state() {
        circuit = circuit.advance();
    }

    println!("The circuit's value is {}.", circuit.output_number());
}

struct Circuit<'a> {
    wire_states: HashMap<&'a str, u8>,
    gates: Vec<Gate<'a>>,
}

impl<'a> Circuit<'a> {
    fn parse(input: &'a str) -> Self {
        let mut wire_states = HashMap::new();
        let mut gates = Vec::new();
        let mut in_wires = true;

        for line in input.lines() {
            if line.is_empty() {
                in_wires = false;
                continue;
            }

            if in_wires {
                let (name, state) = line.split_once(": ").unwrap();
                wire_states.insert(name, state.parse::<u8>().unwrap());
            } else {
                gates.push(Gate::parse(line));
            }
        }

        Self { wire_states, gates }
    }

    fn is_steady_state(&self) -> bool {
        for g in &self.gates {
            if !self.wire_states.contains_key(g.output) {
                return false;
            }
        }

        true
    }

    fn output_number(&self) -> u64 {
        let mut zs = Vec::new();
        for w in self.wire_states.keys() {
            if w.starts_with('z') {
                zs.push(*w);
            }
        }

        zs.sort();
        zs.reverse();

        let mut res = 0;
        for z in zs {
            let state = self.wire_states.get(z).unwrap();
            res = res << 1;
            res = res | u64::from(*state);
        }

        res
    }

    fn advance(&self) -> Self {
        let mut new_wire_states = self.wire_states.clone();

        for gate in &self.gates {
            if self.wire_states.get(gate.output).is_none() {
                if let Some(i1) = self.wire_states.get(gate.input1) {
                    if let Some(i2) = self.wire_states.get(gate.input2) {
                        let o = gate.apply(i1, i2);
                        new_wire_states.insert(gate.output, o);
                    }
                }
            }
        }
        Self {
            wire_states: new_wire_states,
            gates: self.gates.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
    operation: Operation,
}

impl<'a> Gate<'a> {
    fn parse(line: &'a str) -> Self {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        Self {
            input1: parts[0],
            operation: Operation::parse(parts[1]),
            input2: parts[2],
            output: parts[4],
        }
    }

    fn apply(&self, i1: &u8, i2: &u8) -> u8 {
        match self.operation {
            Operation::And => i1 & i2,
            Operation::Or => i1 | i2,
            Operation::Xor => i1 ^ i2,
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        }
    }
}
