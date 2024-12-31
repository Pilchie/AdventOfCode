use std::{collections::HashMap, env, fs, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let robots = args[2].parse::<usize>().expect("Should be a number of robots to use");

    let human_part1 = DPad::human();
    let keypad1 = DPad::keypad("keypad1", human_part1);
    // Keypad 0 is always the NumPad

    let mut keypad_part2 = DPad::human();
    for i in 1..=robots {
        let name = format!("keypad{}", i);
        let keypad_n = DPad::keypad(&name, keypad_part2);
        keypad_part2 = keypad_n;
    }

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in contents.lines() {
        let num = &line[0..3].parse::<usize>().unwrap();
        let code: Vec<_> = line.chars().collect();

        let mut cur = &'A';
        let mut dpad_input = Vec::new();
        for ch in &code {
            dpad_input.extend_from_slice(&NumPad::paths_between(&cur, ch));
            dpad_input.push('A');
            cur = ch;
        }
        let result_part1 = keypad1.keypresses(&dpad_input);
        let result_part2 = keypad_part2.keypresses(&dpad_input);
        //println!("Result for part one is '{}'", string_for(&result_part1));
        //println!("Result for part two is '{}'", string_for(&result_part2));
        let complexity_part1 = num * result_part1.len();
        let complexity_part2 = num * result_part2.len();
        println!(
            "Length for {} is {} for part1 and {} for part 2. Complexities are {} and {}",
            string_for(&code),
            result_part1.len(),
            result_part2.len(),
            complexity_part1,
            complexity_part2
        );

        sum_part1 += complexity_part1;
        sum_part2 += complexity_part2;

        //break;
    }

    println!(
        "The total complexity is {} for part 1, and {} for part 2",
        sum_part1, sum_part2
    );
}

fn string_for(input: &[char]) -> String {
    let mut res = String::new();
    for ch in input {
        res.push(*ch);
    }
    res
}

trait Path {
    fn paths_between(start: &char, end: &char) -> Vec<char>;
}

struct DPad {
    costs: HashMap<(char, char), Vec<char>>,
}

impl DPad {
    fn human() -> Self {
        let mut costs = HashMap::new();
        for s in "^A<v>".chars() {
            for e in "^A<v>".chars() {
                let mut path = Self::paths_between(&s, &e);
                path.push('A');
                costs.insert((s, e), path);
            }
        }
        Self {
            costs,
        }
    }

    fn keypad(_name: &str, parent: DPad) -> Self {
        //println!("Building {}", name);
        let mut costs = HashMap::new();
        for s in "^A<v>".chars() {
            for e in "^A<v>".chars() {
                let mut path = Self::paths_between(&s, &e);
                path.push('A');
                let mut parent_path = Vec::new();
                //print!("  Going from {} - {} is: '{}'", s, e, string_for(&path));
                let mut cur = 'A';
                for next in path {
                    parent_path.extend_from_slice(parent.costs.get(&(cur, next)).unwrap());
                    cur = next;
                }
                //println!(" - keypresses are {:?}, length: {}", string_for(&parent_path), parent_path.len());
                costs.insert((s, e), parent_path);
            }
        }
        Self { costs }
    }

    fn keypresses(&self, code: &[char]) -> Vec<char> {
        let mut res = Vec::new();
        let mut cur = &'A';
        for ch in code {
            let current = self.costs.get(&(*cur, *ch)).unwrap();
            res.extend_from_slice(&current);

            //println!("Going to press {} from {} via is {}", ch, cur, string_for(&current));
            cur = ch;
        }
        res
    }
}

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
                '^' => vec!['>', '^'],      // Skip path through empty space
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
                '0' => vec!['>', 'v'],      // Skip path through blank
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
