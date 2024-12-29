use std::{collections::HashMap, env, fs, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

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

        let complexity_part1 = dpad2_paths.first().unwrap().len();
        println!("{} keypresses", complexity_part1);

        let mut prev = numpad_paths;
        for i in 0..15 {
            let step_paths = possible_paths_for_paths::<DPad>(&prev);
            println!("Completed step {}, with lenght {}", i, step_paths.first().unwrap().len());
            prev = step_paths;
        }

        let complexity_part2 = prev.first().unwrap().len();

        sum_part1 += num * complexity_part1;
        sum_part2 += num * complexity_part2;
    }

    println!(
        "The total complexity is {} for part 1, and {} for part 2",
        sum_part1, sum_part2
    );
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

fn shortest_path<P: Path>(paths: &[Vec<char>], seen: &mut HashMap<Vec<char>, Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for path in paths {
        if path.len() == 0 {
            continue;
        }

        let mut a = 0;
        for i in 0..path.len() {
            if path[i] == 'A' {
                a = i;
            }
        }

        let path_first = &path[0..a];
        let path_rest = &path[a + 1..];
        let rest = &shortest_path::<P>(path_rest, seen);

        if let Some(first) = seen.get(path_first) {
            let mut res = first.clone();
            res.extend_from_slice(rest);
            return  res;
        }

        let possible = possible_paths_for_paths::<P>(&[path_first.to_vec()]);
        let mut m = possible.first().unwrap().clone();
        seen.insert(path.to_vec(), m.clone());
        m.extend_from_slice(rest);
        m
    }

    res
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
        let path_between = P::paths_between(&start, ch);
        for sf in so_far {
            let mut n: Vec<char> = sf.clone();
            n.extend_from_slice(&path_between);
            n.push('A');
            next.push(n);
        }
        start = *ch;
        so_far = next;
    }

    so_far
}

trait Path {
    fn paths_between(start: &char, end: &char) -> Vec<char>;
}

struct DPad {}

impl Path for DPad {
    fn paths_between(start: &char, end: &char) -> Vec<char> {
        match start {
            '^' => match end {
                '^' => vec![],
                'A' => vec!['>'],
                '<' => vec!['v', '<'], // Skip path through empty space
                'v' => vec!['v'],
                '>' => vec!['v', '>'],
                _ => unreachable!(),
            },
            'A' => match end {
                '^' => vec!['<'],
                'A' => vec![],
                '<' => vec!['v', '<', '<'], // Skip path through empty space
                'v' => vec!['<', 'v'],
                '>' => vec!['v'],
                _ => unreachable!(),
            },
            '<' => match end {
                '^' => vec!['>', '^'], // Skip path through empty space
                'A' => vec!['>', '>', '^'], // Skip path through empty space
                '<' => vec![],
                'v' => vec!['>'],
                '>' => vec!['>', '>'],
                _ => unreachable!(),
            },
            'v' => match end {
                '^' => vec!['^'],
                'A' => vec!['^', '>'],
                '<' => vec!['<'],
                'v' => vec![],
                '>' => vec!['>'],
                _ => unreachable!(),
            },
            '>' => match end {
                '^' => vec!['<', '^'],
                'A' => vec!['^'],
                '<' => vec!['<', '<'],
                'v' => vec!['<'],
                '>' => vec![],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

struct NumPad {}

impl Path for NumPad {
    fn paths_between(start: &char, end: &char) -> Vec<char> {
        match start {
            '7' => match end {
                '7' => vec![],
                '8' => vec!['>'],
                '9' => vec!['>', '>'],
                '4' => vec!['v'],
                '5' => vec!['v', '>'],
                '6' => vec!['v', '>', '>'],
                '1' => vec!['v', 'v'],
                '2' => vec!['v', 'v', '>'],
                '3' => vec!['v', 'v', '>', '>'],
                '0' => vec!['>', 'v', 'v', 'v'], // Skip paths through the blank
                'A' => vec!['>', '>', 'v', 'v', 'v'], // Skip paths through the blank
                _ => unreachable!(),
            },
            '8' => match end {
                '7' => vec!['<'],
                '8' => vec![],
                '9' => vec!['>'],
                '4' => vec!['<', 'v'],
                '5' => vec!['v'],
                '6' => vec!['v', '>'],
                '1' => vec!['<', 'v', 'v'],
                '2' => vec!['v', 'v'],
                '3' => vec!['v', 'v', '>'],
                '0' => vec!['v', 'v', 'v'],
                'A' => vec!['v', 'v', 'v', '>'],
                _ => unreachable!(),
            },
            '9' => match end {
                '7' => vec!['<', '<'],
                '8' => vec!['<'],
                '9' => vec![],
                '4' => vec!['<', '<', 'v'],
                '5' => vec!['<', 'v'],
                '6' => vec!['v'],
                '1' => vec!['<', '<', 'v', 'v'],
                '2' => vec!['<', 'v', 'v'],
                '3' => vec!['v', 'v'],
                '0' => vec!['<', 'v', 'v', 'v'],
                'A' => vec!['v', 'v', 'v'],
                _ => unreachable!(),
            },
            '4' => match end {
                '7' => vec!['^'],
                '8' => vec!['^', '>'],
                '9' => vec!['^', '>', '>'],
                '4' => vec![],
                '5' => vec!['>'],
                '6' => vec!['>', '>'],
                '1' => vec!['v'],
                '2' => vec!['v', '>'],
                '3' => vec!['v', '>', '>'],
                '0' => vec!['>', 'v', 'v'], // Skip path through blank
                'A' => vec!['>', '>', 'v', 'v'], // Skip path through blank
                _ => unreachable!(),
            },
            '5' => match end {
                '7' => vec!['<', '^'],
                '8' => vec!['^'],
                '9' => vec!['^', '>'],
                '4' => vec!['<'],
                '5' => vec![],
                '6' => vec!['>'],
                '1' => vec!['<', 'v'],
                '2' => vec!['v'],
                '3' => vec!['v', '>'],
                '0' => vec!['v', 'v'],
                'A' => vec!['v', 'v', '>'],
                _ => unreachable!(),
            },
            '6' => match end {
                '7' => vec!['<', '<', '^'],
                '8' => vec!['<', '^'],
                '9' => vec!['^'],
                '4' => vec!['<', '<'],
                '5' => vec!['<'],
                '6' => vec![],
                '1' => vec!['<', '<', 'v'],
                '2' => vec!['<', 'v'],
                '3' => vec!['v'],
                '0' => vec!['<', 'v', 'v'],
                'A' => vec!['v', 'v'],
                _ => unreachable!(),
            },
            '1' => match end {
                '7' => vec!['^', '^'],
                '8' => vec!['^', '^', '>'],
                '9' => vec!['^', '^', '>', '>'],
                '4' => vec!['^'],
                '5' => vec!['^', '>'],
                '6' => vec!['^', '>', '>'],
                '1' => vec![],
                '2' => vec!['>'],
                '3' => vec!['>', '>'],
                '0' => vec!['>', 'v'], // Skip path through blank
                'A' => vec!['>', '>', 'v'], // Skip path through blank,
                _ => unreachable!(),
            },
            '2' => match end {
                '7' => vec!['<', '^', '^'],
                '8' => vec!['^', '^'],
                '9' => vec!['^', '^', '>'],
                '4' => vec!['<', '^'],
                '5' => vec!['^'],
                '6' => vec!['^', '>'],
                '1' => vec!['<'],
                '2' => vec![],
                '3' => vec!['>'],
                '0' => vec!['v'],
                'A' => vec!['v', '>'],
                _ => unreachable!(),
            },
            '3' => match end {
                '7' => vec!['<', '<', '^', '^'],
                '8' => vec!['<', '^', '^'],
                '9' => vec!['^', '^'],
                '4' => vec!['<', '<', '^'],
                '5' => vec!['<', '^'],
                '6' => vec!['^'],
                '1' => vec!['<', '<'],
                '2' => vec!['<'],
                '3' => vec![],
                '0' => vec!['<', 'v'],
                'A' => vec!['v'],
                _ => unreachable!(),
            },
            '0' => match end {
                '7' => vec!['^', '^', '^', '<'], // Skip path through blank
                '8' => vec!['^', '^', '^'],
                '9' => vec!['^', '^', '^', '>'],
                '4' => vec!['^', '^', '<'], // Skip path through blank
                '5' => vec!['^', '^'],
                '6' => vec!['^', '^', '>'],
                '1' => vec!['^', '<'], // Skip path through blank
                '2' => vec!['^'],
                '3' => vec!['^', '>'],
                '0' => vec![],
                'A' => vec!['>'],
                _ => unreachable!(),
            },
            'A' => match end {
                '7' => vec!['^', '^', '^', '<', '<'], // Skip path through blank
                '8' => vec!['<', '^', '^', '^'],
                '9' => vec!['^', '^', '^'],
                '4' => vec!['^', '^', '<', '<'], // Skip path through blank
                '5' => vec!['<', '^', '^'],
                '6' => vec!['^', '^'],
                '1' => vec!['^', '<', '<'], // Skip path through blank
                '2' => vec!['<', '^'],
                '3' => vec!['^'],
                '0' => vec!['<'],
                'A' => vec![],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
