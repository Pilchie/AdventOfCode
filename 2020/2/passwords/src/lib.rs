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

    pub fn is_valid(&self) -> bool {
        let mut count: i32 = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }
}