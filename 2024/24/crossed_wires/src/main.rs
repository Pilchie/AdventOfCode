use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let circuit = Circuit::parse(&contents);
    let part1 = circuit.run();
    println!("The circuit's value is {}.", part1.output_number());

    // Try to find the wrong outputs.
    // Start by figuring out what bits they are in.
    let bad_bits = find_bad_bits(&circuit);
    println!("Bad bits are: {:?}", bad_bits);

    // Add all the gates that have those x and y's as inputs.
    let mut possible_gates = Vec::new();
    let mut initial_level = HashSet::new();
    for gate in &circuit.gates {
        for b in &bad_bits {
            let xstr = format!("x{:02}", b);
            let ystr = format!("y{:02}", b);
            if gate.input1 == xstr
                || gate.input2 == xstr
                || gate.input1 == ystr
                || gate.input2 == ystr
            {
                initial_level.insert(gate);
            }
        }
    }
    println!(
        "There are {} gates that have bad bits as direct inputs",
        initial_level.len()
    );
    possible_gates.push(initial_level.clone());


    let mut previous_level = initial_level;
    loop {
        let mut next_level = HashSet::new();
        for c in previous_level {
            for g in &circuit.gates{ 
                if c.output == g.input1 || c.output == g.input2 {
                    next_level.insert(g);
                }
            } 
        };
        if next_level.is_empty() {
            break;
        }
        possible_gates.push(next_level.clone());
        previous_level = next_level;
    }

    println!("There are {} levels to explore", possible_gates.len());
    let mut outputs_to_swap = HashSet::new();
    for level in &possible_gates {
        println!("Trying a new level with {} elements", level.len());
        for g1 in level {
            for g2 in level {
                if g1 != g2 {
                    //println!("  Trying to swap {} and {}", g1.output, g2.output);
                    let modified = circuit.swap_outputs(g1, g2);
                    if !modified.has_cycle() {
                        if find_bad_bits(&modified).len() < bad_bits.len() {
                            println!("  Found an improvement by swapping {} and {}", g1.output, g2.output);
                            outputs_to_swap.insert((g1, g2));
                        }
                    }
                }
            }
        }
    }

    let mut fixed = circuit.clone();
    let mut wires = Vec::new();
    for (g1, g2) in outputs_to_swap {
        fixed = fixed.swap_outputs(g1, g2);
        wires.push(g1.output);
        wires.push(g2.output);
    }

    let fixed_bad = find_bad_bits(&fixed);
    println!("There are {} bad bits in the fixed version", fixed_bad.len());

    wires.sort();
    wires.dedup();
    println!("The wires are: {}", wires.join(","));

}

fn find_bad_bits(circuit: &Circuit) -> Vec<u8> {
    let mut bad_bits = Vec::new();
    for i in 0..45 {
        if !test_bit(&circuit, i) {
            bad_bits.push(i);
        }
    }
    bad_bits
}

fn test_bit(circuit: &Circuit, bit: u8) -> bool {
    test_bit_with(&circuit, bit, 0, 0, 0, 0)
        && test_bit_with(&circuit, bit, 0, 1, 1, 0)
        && test_bit_with(&circuit, bit, 1, 0, 1, 0)
        && test_bit_with(&circuit, bit, 1, 1, 0, 1)
}

fn test_bit_with(circuit: &Circuit, bit: u8, x: u8, y: u8, z: u8, zc: u8) -> bool {
    let mut wires: HashMap<&str, u8> = HashMap::new();
    for i in 0..45 {
        let xstr = format!("x{:02}", i);
        let ystr = format!("y{:02}", i);
        let xkey = circuit.wire_states.keys().find(|k| *k == &xstr).unwrap();
        let ykey = circuit.wire_states.keys().find(|k| *k == &ystr).unwrap();

        if i == bit {
            wires.insert(xkey, x);
            wires.insert(ykey, y);
        } else {
            wires.insert(xkey, 0);
            wires.insert(ykey, 0);
        }
    }
    let circuit = circuit.with_wires(wires);
    let output = circuit.run().output_number();
    let expected = u64::from(zc) << bit + 1 | u64::from(z) << bit;
    let matches = output == expected;
    // if !matches {
    //     println!("Tried bit:{}, with x:{}, y:{}. Expected zc:{}, z:{}, expected: {}, output: {} - matches: {}",
    //     bit, x, y, zc, z, expected, output, matches);
    // }
    matches
}

#[derive(Clone)]
struct Circuit<'a> {
    wire_states: HashMap<&'a str, u8>,
    gates: HashSet<Gate<'a>>,
}

impl<'a> Circuit<'a> {
    fn parse(input: &'a str) -> Self {
        let mut wire_states = HashMap::new();
        let mut gates = HashSet::new();
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
                gates.insert(Gate::parse(line));
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

    fn run(&self) -> Self {
        let mut circuit = self.clone();
        while !circuit.is_steady_state() {
            circuit = circuit.advance();
        }
        circuit
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

    fn with_wires(&self, wires: HashMap<&'a str, u8>) -> Self {
        Self {
            wire_states: wires,
            gates: self.gates.clone(),
        }
    }

    fn swap_outputs(&self, g1: &'a Gate, g2: &'a Gate) -> Self {
        let mut gates = self.gates.clone();
        gates.remove(g1);
        gates.remove(g2);
        gates.insert(Gate {
            input1: g1.input1,
            input2: g1.input2,
            operation: g1.operation,
            output: g2.output,
        });
        gates.insert(Gate {
            input1: g2.input1,
            input2: g2.input2,
            operation: g2.operation,
            output: g1.output,
        });
        Self {
            wire_states: self.wire_states.clone(),
            gates,
        }
    }

    fn has_cycle(&self) -> bool {
        for i in 0..45 {
            let xstr = format!("x{:02}", i);
            let ystr = format!("y{:02}", i);

            if self.wire_has_cycle(&xstr) {
                return true;
            }
            if self.wire_has_cycle(&ystr) {
                return true;
            }
        }

        false
    }

    fn wire_has_cycle(&self, wire: &str) -> bool {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(wire);

        while !(queue.is_empty())
        {
            let cur = queue.pop_front().unwrap();
            for g in &self.gates {
                if g.input1 == cur || g.input2 == cur {
                    if seen.contains(&g) {
                        println!("found a cycle involving {} - {:?}", wire, g);
                        return true;
                    }
                    seen.insert(g);
                    queue.push_back(&g.output);
                }
            }
        }

        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
