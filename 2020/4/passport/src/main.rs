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
            current = current + &l;
        }
    }

    println!("Found {} total passports", passports.len());
    println!("Found {} valid passports", passports.iter().filter(|p| p.is_valid()).count());
}

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
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
                    birth_year = Some(String::from(value));
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
            country_id: country_id,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.birth_year {
            Some(_) => match self.issue_year {
                Some(_) => match self.expiration_year {
                    Some(_) => match self.height {
                        Some(_) => match self.hair_color {
                            Some(_) => match self.eye_color {
                                Some(_) => match self.passport_id {
                                    Some(_) => true,
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

#[test]
fn is_valid_passport_first() {
    let passport = Passport::from_string(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    assert_eq!(true, passport.is_valid());
}

#[test]
fn is_valid_passport_second() {
    let passport = Passport::from_string(
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929",
    );

    assert_eq!(false, passport.is_valid());
}

#[test]
fn is_valid_passport_third() {
    let passport = Passport::from_string(
        "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm",
    );

    assert_eq!(true, passport.is_valid());
}

#[test]
fn is_valid_passport_fourth() {
    let passport = Passport::from_string(
        "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in",
    );

    assert_eq!(false, passport.is_valid());
}
