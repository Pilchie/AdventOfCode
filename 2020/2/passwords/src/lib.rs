#[derive(Debug)]
pub struct PasswordLine {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

impl PasswordLine {
    pub fn parse(line: &str) -> PasswordLine {
        let mut chars = line.chars();
        let mut start = 0;
        let mut end = 0;

        while chars.next().unwrap() != '-' {
            end += 1;
        }
        let min = line[start..end].parse::<i32>().unwrap();

        // Skip the -
        end += 1;
        start = end;

        while chars.next().unwrap() != ' ' {
            end += 1;
        }
        let max = line[start..end].parse::<i32>().unwrap();

        // Skip the ' '
        end += 1;
        start = end;

        let rule_char = chars.next().unwrap();

        // Skip the "c: "
        start += 3;
        let password = &line[start..];

        PasswordLine {
            min: min,
            max: max,
            letter: rule_char,
            password: password.to_string(),
        }
    }

    pub fn is_valid_part1(&self) -> bool {
        let mut count: i32 = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }

    pub fn is_valid_part2(&self) -> bool {
        let charmin = self.password.chars().nth((self.min - 1) as usize).unwrap();
        let charmax = self.password.chars().nth((self.max - 1) as usize).unwrap();

        println!("letter: {}, min: {}, max: {}, password: {}, charmin: {}, charmax: {}",
            self.letter, self.min, self.max, self.password, charmin, charmax);

        charmin == self.letter && charmax != self.letter
            || charmin != self.letter && charmax == self.letter
    }
}
