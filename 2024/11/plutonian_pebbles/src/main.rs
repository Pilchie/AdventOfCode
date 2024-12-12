use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let stones: Vec<u64> = contents
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    println!("Initial arrangement:");
    println!("{:?}", stones);

    let mut known = HashMap::<State, u64>::new();
    let mut count = 0;
    for s in stones {
        count += count_children_of(s, 75, &mut known);
    }
    println!("{}", count);
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    val: u64,
    blinks: u8,
}

fn count_children_of(val: u64, blinks: u8, known: &mut HashMap<State, u64>) -> u64 {
    if let Some(val) = known.get(&State { val, blinks }) {
        return *val;
    }

    let sstr = val.to_string();
    let res: u64;
    if val == 0 {
        res = match blinks > 1 {
            true => count_children_of(1, blinks - 1, known),
            false => 1,
        };
    } else if sstr.len() % 2 == 0 {
        res = match blinks > 1 {
            true => {
                let v1 = sstr[0..sstr.len() / 2].parse::<u64>().unwrap();
                let v2 = sstr[sstr.len() / 2..].parse::<u64>().unwrap();
                let res1 = count_children_of(v1, blinks - 1, known);
                let res2 = count_children_of(v2, blinks - 1, known);
                res1 + res2
            },
            false => 2,
        };
    } else {
        res = match blinks > 1 {
            true => count_children_of(val * 2024, blinks - 1, known),
            false => 1,
        };
    }

    known.insert(
        State {
            val,
            blinks,
        },
        res,
    );

    res
}
