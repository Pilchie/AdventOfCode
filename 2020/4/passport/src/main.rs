use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]);

    let reader = io::BufReader::new(file.unwrap());
    let mut current = String::from("");
    let mut passports = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            let passport = Passport::from_string(&current);
            passports.push(passport);
            current = String::from("");
        } else {
            current = current + " " + &l;
        }
    }
    let passport = Passport::from_string(&current);
    passports.push(passport);

    println!("Found {} total passports", passports.len());
    println!(
        "Found {} valid passports",
        passports.iter().filter(|p| p.is_valid()).count()
    );
}

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    _country_id: Option<String>, // Uneeded to solve problem
}

impl Passport {
    pub fn from_string(data: &str) -> Passport {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;
        let parts = data.split_whitespace();
        for p in parts {
            let mut items = p.split(':');
            let name = items.next().unwrap();
            let value = items.next().unwrap();
            match name {
                "byr" => {
                    birth_year = Some(value.into());
                }
                "iyr" => {
                    issue_year = Some(String::from(value));
                }
                "eyr" => {
                    expiration_year = Some(String::from(value));
                }
                "hgt" => {
                    height = Some(String::from(value));
                }
                "hcl" => {
                    hair_color = Some(String::from(value));
                }
                "ecl" => {
                    eye_color = Some(String::from(value));
                }
                "pid" => {
                    passport_id = Some(String::from(value));
                }
                "cid" => {
                    country_id = Some(String::from(value));
                }

                &_ => {}
            }
        }
        Passport {
            birth_year: birth_year,
            issue_year: issue_year,
            expiration_year: expiration_year,
            height: height,
            hair_color: hair_color,
            eye_color: eye_color,
            passport_id: passport_id,
            _country_id: country_id,
        }
    }

    pub fn is_valid(&self) -> bool {
        match &self.birth_year {
            Some(byr) => is_valid_year(&byr, 1920, 2002) && match &self.issue_year {
                Some(iyr) => is_valid_year(&iyr, 2010, 2020) && match &self.expiration_year {
                    Some(eyr) => is_valid_year(&eyr, 2020, 2030) && match &self.height {
                        Some(h) => is_valid_height(h) && match &self.hair_color {
                            Some(hcl) => is_valid_hair_color(hcl) && match &self.eye_color {
                                Some(ecl) => is_valid_eye_color(ecl) && match &self.passport_id {
                                    Some(pid) => is_valid_passport_id(pid),
                                    None => false,
                                },
                                None => false,
                            },
                            None => false,
                        },
                        None => false,
                    },
                    None => false,
                },
                None => false,
            },
            None => false,
        }
    }
}

fn is_valid_year(value: &str, min: usize, max: usize) -> bool {
    if value.len() != 4 {
        return false;
    }

    let v = value.parse::<usize>().unwrap();
    min <= v && v <= max
}

fn is_valid_hair_color(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }

    let mut chars = value.chars();
    if chars.next().unwrap() != '#' {
        return false;
    }

    for _ in 1..6 {
        if !chars.next().unwrap().is_digit(16) {
            return false;
        }
    }
    true
}

fn is_valid_eye_color(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn is_valid_passport_id(value: &str) -> bool {
    value.len() == 9 && value.chars().all(|c| c.is_digit(10))
}

fn is_valid_height(value: &str) -> bool {
    if value.ends_with(&"cm") {
        return is_between(value, 150, 193);
    } else if value.ends_with(&"in") {
        return is_between(value, 59, 76);
    }

    false
}

fn is_between(value: &str, min: usize, max: usize) -> bool {
    let len = value.len();
    let v = value[0..len - 2].parse::<usize>();
    match v {
        Ok(vv) => vv >= min && vv <= max,
        Err(_) => false,
    }
}

#[test]
fn is_valid_passport_first() {
    let passport = Passport::from_string(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    assert!(passport.is_valid());
}

#[test]
fn is_valid_passport_second() {
    let passport = Passport::from_string(
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929",
    );

    assert!(!passport.is_valid());
}

#[test]
fn is_valid_passport_third() {
    let passport = Passport::from_string(
        "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm",
    );

    assert!(passport.is_valid());
}

#[test]
fn is_valid_passport_fourth() {
    let passport = Passport::from_string(
        "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in",
    );

    assert!(!passport.is_valid());
}

#[test]
fn test_height_60in() {
    assert!(is_valid_height("60in"));
}

#[test]
fn test_height_190cm() {
    assert!(is_valid_height("190cm"));
}

#[test]
fn test_height_190in() {
    assert!(!is_valid_height("190in"));
}

#[test]
fn test_height_190() {
    assert!(!is_valid_height("190"));
}
