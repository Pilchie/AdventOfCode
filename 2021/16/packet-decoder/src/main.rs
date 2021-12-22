fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let line = match args.len() > 2 {
        true => args[2].parse::<usize>().unwrap(),
        false => 0,
    };

    let mut bits = Vec::new();
    bits.reserve_exact(input.lines().nth(line).unwrap().len());
    for b in input.lines().nth(line).unwrap().bytes() {
        match b {
            b'0' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
            },
            b'1' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
            },
            b'2' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
            },
            b'3' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
            },
            b'4' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
            },
            b'5' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
            },
            b'6' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
            },
            b'7' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
            },
            b'8' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
            },
            b'9' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
            },
            b'A' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
            },
            b'B' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
            },
            b'C' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
            },
            b'D' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
            },
            b'E' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
            },
            b'F' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
            },
            _ => panic!("Unexpected byte: '{}'", b),
        };
    }

    let mut index = 0usize;
    let mut total = 0;
    index = parse_packet(index, &bits, &mut total);

    for i in index..bits.len() {
        if bits[i] != 0 {
            panic!("After packet, but non-zero bits at {}?", i);
        }
    }

    println!("The total is: {}", total);

    Ok(())
}

fn parse_packet(index: usize, bits: &Vec<u8>, total: &mut i32) -> usize {
    println!("parsing packet at {}", index);

    let mut i = index;
    let packet_version = parse(&mut i, bits, 3);
    *total += packet_version;

    let packet_type = parse(&mut i, bits, 3);

    match packet_type {
        4 => parse_literal_body(&mut i, &bits),
        _ => parse_operator_body(&mut i, &bits, total),
    }

    i
}

fn parse_literal_body(i: &mut usize, bits: &Vec<u8>) {
    println!("parsing literal at {}", i);
    while bits[*i] == 1 {
        *i += 5;
    }

    *i += 5;
}

fn parse_operator_body(i: &mut usize, bits: &Vec<u8>, total: &mut i32) {
    println!("parsing operator body at {}", i);

    let length_type = bits[*i];
    *i += 1;

    match length_type {
        0 => parse_packets_length(i, bits, total),
        1 => parse_packets_count(i, bits, total),
        _ => panic!("Unexpected length bit! {}", length_type),
    };
}

fn parse_packets_length(i: &mut usize, bits: &Vec<u8>, total: &mut i32) {
    let length = parse(i, bits, 15);

    let start = *i;
    while *i < start + length as usize {
        *i = parse_packet(*i, bits, total);
    }
}

fn parse_packets_count(i: &mut usize, bits: &Vec<u8>, total: &mut i32) {
    let count = parse(i, bits, 11);

    for _ in 0..count {
        *i = parse_packet(*i, bits, total);
    }
}

fn parse(i: &mut usize, bits: &Vec<u8>, count: usize) -> i32 {
    let mut value = bits[*i] as i32;
    for b in 1..count {
        value = (value << 1) + bits[*i + b] as i32;
    }

    *i += count;
    value
}
