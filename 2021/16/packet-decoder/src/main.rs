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
            }
            b'1' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
            }
            b'2' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
            }
            b'3' => {
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
            }
            b'4' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
            }
            b'5' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
            }
            b'6' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
            }
            b'7' => {
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
            }
            b'8' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(0u8);
            }
            b'9' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
                bits.push(1u8);
            }
            b'A' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(0u8);
            }
            b'B' => {
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
                bits.push(1u8);
            }
            b'C' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(0u8);
            }
            b'D' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
                bits.push(1u8);
            }
            b'E' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(0u8);
            }
            b'F' => {
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
                bits.push(1u8);
            }
            _ => panic!("Unexpected byte: '{}'", b),
        };
    }

    let mut index = 0usize;
    let res = parse_packet(&mut index, &bits);

    for i in index..bits.len() {
        if bits[i] != 0 {
            panic!("After packet, but non-zero bits at {}?", i);
        }
    }

    println!("The result is: {}", res);

    Ok(())
}

fn parse_packet(i: &mut usize, bits: &Vec<u8>) -> i64 {
    print!("parsing packet at {} ->", i);

    let _packet_version = parse(i, bits, 3);
    let packet_type = parse(i, bits, 3);

    let res = match packet_type {
        4 => parse_literal_body(i, &bits),
        _ => parse_operator_body(i, &bits, packet_type),
    };

    println!("{}", res);

    res
}

fn parse_literal_body(i: &mut usize, bits: &Vec<u8>) -> i64 {
    let mut val = 0;
    while bits[*i] == 1 {
        *i += 1;
        let chunk = parse(i, bits, 4);
        val = (val << 4) + chunk;
    }

    *i += 1;
    let chunk = parse(i, bits, 4);
    val = (val << 4) + chunk;

    val
}

fn parse_operator_body(i: &mut usize, bits: &Vec<u8>, packet_type: i64) -> i64 {
    let length_type = bits[*i];
    *i += 1;

    let vals = match length_type {
        0 => parse_packets_length(i, bits),
        1 => parse_packets_count(i, bits),
        _ => panic!("Unexpected length bit! {}", length_type),
    };

    match packet_type {
        0 => vals.iter().fold(0, |a, v| a + v),
        1 => vals.iter().fold(1, |a, v| a * v),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => match vals[0] > vals[1] {
            true => 1,
            false => 0,
        },
        6 => match vals[0] < vals[1] {
            true => 1,
            false => 0,
        },
        7 => match vals[0] == vals[1] {
            true => 1,
            false => 0,
        },
        _ => panic!("Unexpected packet type {}", packet_type),
    }
}

fn parse_packets_length(i: &mut usize, bits: &Vec<u8>) -> Vec<i64> {
    let length = parse(i, bits, 15);

    let start = *i;
    let mut res = Vec::new();
    while *i < start + length as usize {
        res.push(parse_packet(i, bits));
    }

    res
}

fn parse_packets_count(i: &mut usize, bits: &Vec<u8>) -> Vec<i64> {
    let count = parse(i, bits, 11);

    let mut res = Vec::new();
    for _ in 0..count {
        res.push(parse_packet(i, bits));
    }

    res
}

fn parse(i: &mut usize, bits: &Vec<u8>, count: usize) -> i64 {
    let mut value = bits[*i] as i64;
    for b in 1..count {
        value = (value << 1) + bits[*i + b] as i64;
    }

    *i += count;
    value
}
