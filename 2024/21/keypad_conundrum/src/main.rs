use std::{env, fs, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        let num = &line[0..3].parse::<usize>().unwrap();
        let chars: Vec<_> = line.chars().collect();
        print!("Generating keypresses for {} - ", line);

        let numpad_paths = possible_paths_for_code::<NumPad>(&chars);
        //_dump("numpad_paths", &numpad_paths);
        let dpad1_paths = possible_paths_for_paths::<DPad>(&numpad_paths);
        //_dump("dpad1_paths", &dpad1_paths);
        let dpad2_paths = possible_paths_for_paths::<DPad>(&dpad1_paths);
        //_dump("dpad2_paths", &dpad2_paths);

        let complexity = dpad2_paths.first().unwrap().len();
        println!("{} keypresses", complexity);

        sum += num * complexity;
    }

    println!("The total complexity is {}", sum);
}

fn _dump(label: &str, paths: &[Vec<char>]) {
    println!("Dumping {}", label);
    for vec in paths {
        print!("  ");
        for ch in vec {
            print!("{}", ch);
        }
        println!();
    }
}

fn possible_paths_for_paths<P: Path>(input_paths: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut min = usize::MAX;
    let mut result = Vec::new();
    for input_path in input_paths {
        let output_paths = possible_paths_for_code::<P>(&input_path);
        for output_path in output_paths {
            if output_path.len() < min {
                min = output_path.len();
                result.clear();
                result.push(output_path);
            } else if output_path.len() == min {
                result.push(output_path);
            } else {
                // skip it, it's longer than what we've seen already.
            }
        }
    }
    result
}

fn possible_paths_for_code<P: Path>(code: &[char]) -> Vec<Vec<char>> {
    let mut so_far: Vec<Vec<char>> = vec![vec![]];
    let mut start = 'A';
    for ch in code {
        let mut next = Vec::new();
        let paths_between = P::paths_between(&start, ch);
        for sf in so_far {
            for p in &paths_between {
                let mut n = sf.clone();
                n.extend_from_slice(&p);
                n.push('A');
                next.push(n);
            }
        }
        start = *ch;
        so_far = next;
    }

    so_far
}

trait Path {
    fn paths_between(start: &char, end: &char) -> Vec<Vec<char>>;
}

struct DPad {}

impl Path for DPad {
    fn paths_between(start: &char, end: &char) -> Vec<Vec<char>> {
        match start {
            '^' => match end {
                '^' => vec![vec![]],
                'A' => vec![vec!['>']],
                '<' => vec![vec!['v', '<']],
                'v' => vec![vec!['v']],
                '>' => vec![vec!['v', '>'], vec!['>', 'v']],
                _ => unreachable!(),
            },
            'A' => match end {
                '^' => vec![vec!['<']],
                'A' => vec![vec![]],
                '<' => vec![vec!['v', '<', '<']],
                'v' => vec![vec!['v', '<'], vec!['<', 'v']],
                '>' => vec![vec!['v']],
                _ => unreachable!(),
            },
            '<' => match end {
                '^' => vec![vec!['>', '^'], vec!['^', '<']],
                'A' => vec![vec!['>', '>', '^']],
                '<' => vec![vec![]],
                'v' => vec![vec!['>']],
                '>' => vec![vec!['>', '>']],
                _ => unreachable!(),
            },
            'v' => match end {
                '^' => vec![vec!['^']],
                'A' => vec![vec!['^', '>'], vec!['>', '^']],
                '<' => vec![vec!['<']],
                'v' => vec![vec![]],
                '>' => vec![vec!['>']],
                _ => unreachable!(),
            },
            '>' => match end {
                '^' => vec![vec!['^', '<'], vec!['<', '^']],
                'A' => vec![vec!['^']],
                '<' => vec![vec!['<', '<']],
                'v' => vec![vec!['<']],
                '>' => vec![vec![]],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

struct NumPad {}

impl Path for NumPad {
    fn paths_between(start: &char, end: &char) -> Vec<Vec<char>> {
        match start {
            '7' => match end {
                '7' => vec![vec![]],
                '8' => vec![vec!['>']],
                '9' => vec![vec!['>', '>']],
                '4' => vec![vec!['v']],
                '5' => vec![vec!['v', '>'], vec!['v', '>']],
                '6' => vec![vec!['v', '>', '>'], vec!['>', '>', 'v']],
                '1' => vec![vec!['v', 'v']],
                '2' => vec![vec!['v', 'v', '>'], vec!['>', 'v', 'v']],
                '3' => vec![vec!['v', 'v', '>', '>'], vec!['>', '>', 'v', 'v']],
                '0' => vec![vec!['>', 'v', 'v', 'v']], // Skip paths through the blank
                'A' => vec![vec!['>', '>', 'v', 'v', 'v']], // Skip paths through the blank
                _ => unreachable!(),
            },
            '8' => match end {
                '7' => vec![vec!['<']],
                '8' => vec![vec![]],
                '9' => vec![vec!['>']],
                '4' => vec![vec!['v', '<'], vec!['<', 'v']],
                '5' => vec![vec!['v']],
                '6' => vec![vec!['v', '>'], vec!['>', 'v']],
                '1' => vec![vec!['v', 'v', '<'], vec!['<', 'v', 'v']],
                '2' => vec![vec!['v', 'v']],
                '3' => vec![vec!['v', 'v', '>'], vec!['>', 'v', 'v']],
                '0' => vec![vec!['v', 'v', 'v']],
                'A' => vec![vec!['v', 'v', 'v', '>'], vec!['>', 'v', 'v', 'v']],
                _ => unreachable!(),
            },
            '9' => match end {
                '7' => vec![vec!['<', '<']],
                '8' => vec![vec!['<']],
                '9' => vec![vec![]],
                '4' => vec![vec!['<', '<', 'v'], vec!['v', '<', '<']],
                '5' => vec![vec!['<', 'v']],
                '6' => vec![vec!['v']],
                '1' => vec![vec!['<', '<', 'v', 'v'], vec!['v', 'v', '<', '<']],
                '2' => vec![vec!['<', 'v', 'v'], vec!['v', 'v', '<']],
                '3' => vec![vec!['v', 'v']],
                '0' => vec![vec!['<', 'v', 'v', 'v'], vec!['v', 'v', 'v', '<']],
                'A' => vec![vec!['v', 'v', 'v']],
                _ => unreachable!(),
            },
            '4' => match end {
                '7' => vec![vec!['^']],
                '8' => vec![vec!['^', '>'], vec!['>', '^']],
                '9' => vec![vec!['^', '>', '>'], vec!['>', '>', '^']],
                '4' => vec![vec![]],
                '5' => vec![vec!['>']],
                '6' => vec![vec!['>', '>']],
                '1' => vec![vec!['v']],
                '2' => vec![vec!['v', '>'], vec!['>', 'v']],
                '3' => vec![vec!['v', '>', '>'], vec!['>', '>', 'v']],
                '0' => vec![vec!['>', 'v', 'v']], // Skip path throug blank
                'A' => vec![vec!['>', '>', 'v', 'v']], // Skip path throug blank
                _ => unreachable!(),
            },
            '5' => match end {
                '7' => vec![vec!['^', '<'], vec!['<', '^']],
                '8' => vec![vec!['^']],
                '9' => vec![vec!['^', '>'], vec!['>', '^']],
                '4' => vec![vec!['<']],
                '5' => vec![vec![]],
                '6' => vec![vec!['>']],
                '1' => vec![vec!['<', 'v'], vec!['v', '<']],
                '2' => vec![vec!['v']],
                '3' => vec![vec!['v', '>'], vec!['>', 'v']],
                '0' => vec![vec!['v', 'v']],
                'A' => vec![vec!['v', 'v', '>'], vec!['>', 'v', 'v']],
                _ => unreachable!(),
            },
            '6' => match end {
                '7' => vec![vec!['^', '<', '<'], vec!['<', '<', '^']],
                '8' => vec![vec!['^', '<'], vec!['<', '^']],
                '9' => vec![vec!['^']],
                '4' => vec![vec!['<', '<']],
                '5' => vec![vec!['<']],
                '6' => vec![vec![]],
                '1' => vec![vec!['v', '<', '<'], vec!['<', '<', 'v']],
                '2' => vec![vec!['v', '<'], vec!['<', 'v']],
                '3' => vec![vec!['v']],
                '0' => vec![vec!['<', 'v', 'v'], vec!['v', 'v', '<']],
                'A' => vec![vec!['v', 'v']],
                _ => unreachable!(),
            },
            '1' => match end {
                '7' => vec![vec!['^', '^']],
                '8' => vec![vec!['^', '^', '>'], vec!['>', '^', '^']],
                '9' => vec![vec!['^', '^', '>', '>'], vec!['>', '>', '^', '^']],
                '4' => vec![vec!['^']],
                '5' => vec![vec!['^', '>'], vec!['>', '^']],
                '6' => vec![vec!['^', '>', '>'], vec!['>', '>', '^']],
                '1' => vec![vec![]],
                '2' => vec![vec!['>']],
                '3' => vec![vec!['>', '>']],
                '0' => vec![vec!['>', 'v']], // Skip path through blank
                'A' => vec![vec!['>', '>', 'v']], // Skip path through blank,
                _ => unreachable!(),
            },
            '2' => match end {
                '7' => vec![vec!['<', '^', '^'], vec!['^', '^', '<']],
                '8' => vec![vec!['^', '^']],
                '9' => vec![vec!['^', '^', '>'], vec!['>', '^', '^']],
                '4' => vec![vec!['<', '^'], vec!['^', '<']],
                '5' => vec![vec!['^']],
                '6' => vec![vec!['^', '>'], vec!['>', '^']],
                '1' => vec![vec!['<']],
                '2' => vec![vec![]],
                '3' => vec![vec!['>']],
                '0' => vec![vec!['v']],
                'A' => vec![vec!['v', '>'], vec!['>', 'v']],
                _ => unreachable!(),
            },
            '3' => match end {
                '7' => vec![vec!['<', '<', '^', '^'], vec!['^', '^', '<', '<']],
                '8' => vec![vec!['<', '^', '^'], vec!['^', '<', '^']],
                '9' => vec![vec!['^', '^']],
                '4' => vec![vec!['<', '<', '^'], vec!['^', '<', '<']],
                '5' => vec![vec!['<', '^'], vec!['^', '<']],
                '6' => vec![vec!['^']],
                '1' => vec![vec!['<', '<']],
                '2' => vec![vec!['<']],
                '3' => vec![vec![]],
                '0' => vec![vec!['<', 'v'], vec!['v', '<']],
                'A' => vec![vec!['v']],
                _ => unreachable!(),
            },
            '0' => match end {
                '7' => vec![vec!['^', '^', '^', '<']], // Skip path through blank
                '8' => vec![vec!['^', '^', '^']],
                '9' => vec![vec!['^', '^', '^', '>'], vec!['>', '^', '^', '^']],
                '4' => vec![vec!['^', '^', '<']], // Skip path through blank
                '5' => vec![vec!['^', '^']],
                '6' => vec![vec!['^', '^', '>'], vec!['>', '^', '^']],
                '1' => vec![vec!['^', '<']], // Skip path through blank
                '2' => vec![vec!['^']],
                '3' => vec![vec!['^', '>'], vec!['>', '^']],
                '0' => vec![vec![]],
                'A' => vec![vec!['>']],
                _ => unreachable!(),
            },
            'A' => match end {
                '7' => vec![vec!['^', '^', '^', '<', '<']], // Skip path through blank
                '8' => vec![vec!['^', '^', '^', '<'], vec!['<', '^', '^', '^']],
                '9' => vec![vec!['^', '^', '^']],
                '4' => vec![vec!['^', '^', '<', '<']], // Skip path through blank
                '5' => vec![vec!['^', '^', '<'], vec!['<', '^', '^']],
                '6' => vec![vec!['^', '^']],
                '1' => vec![vec!['^', '<', '<']], // Skip path through blank
                '2' => vec![vec!['^', '<'], vec!['<', '^']],
                '3' => vec![vec!['^']],
                '0' => vec![vec!['<']],
                'A' => vec![vec![]],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
