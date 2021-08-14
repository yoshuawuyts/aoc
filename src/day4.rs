//! https://adventofcode.com/2020/day/4

use std::io::{self, BufRead};

// const INPUT: &'static str = include_str!("../inputs/day4_ex1.txt");
const INPUT: &'static str = include_str!("../inputs/day4.txt");

fn main() -> io::Result<()> {
    let mut passport = Some(Passport::default());
    let mut count = 0;
    let mut validate_passport = |passport: &Option<Passport>| {
        if let Some(passport) = passport {
            if passport.validate() {
                count += 1;
            }
        }
    };
    for (i, line) in io::Cursor::new(INPUT).lines().enumerate() {
        let line = line?;
        if line == "" {
            validate_passport(&passport);
            passport = None;
        } else {
            for field in line.split_whitespace() {
                let passport = passport.get_or_insert_with(|| Passport::new(i));
                let (key, value) = field.split_once(':').unwrap();
                match key {
                    "byr" => passport.birth_year = Some(value.into()),
                    "iyr" => passport.issue_year = Some(value.into()),
                    "eyr" => passport.expiration_year = Some(value.into()),
                    "hgt" => passport.height = Some(value.into()),
                    "hcl" => passport.hair_color = Some(value.into()),
                    "ecl" => passport.eye_color = Some(value.into()),
                    "pid" => passport.passport_id = Some(value.into()),
                    "cid" => passport.country_id = Some(value.into()),
                    _ => panic!("malformed passport input"),
                }
            }
        }
    }
    validate_passport(&passport);
    println!("{}", count);
    Ok(())
}

#[derive(Default, Debug)]
struct Passport {
    id: usize,
    issue_year: Option<String>,
    birth_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    fn validate(&self) -> bool {
        self.validate_issue_year()
            && self.validate_birth_year()
            && self.validate_expiration_year()
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_passport_id()
    }

    fn validate_birth_year(&self) -> bool {
        validate_year(&self.birth_year, 1920, 2002)
    }

    fn validate_issue_year(&self) -> bool {
        validate_year(&self.issue_year, 2010, 2020)
    }

    fn validate_expiration_year(&self) -> bool {
        validate_year(&self.expiration_year, 2020, 2030)
    }

    fn validate_height(&self) -> bool {
        match &self.height {
            Some(s) => {
                if s.len() < 2 {
                    return false;
                }
                match s.split_at(s.len() - 2) {
                    (s, "cm") => {
                        let n = s.parse::<u16>().unwrap();
                        n >= 150 && n <= 193
                    }
                    (s, "in") => {
                        let n = s.parse::<u16>().unwrap();
                        n >= 59 && n <= 76
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn validate_hair_color(&self) -> bool {
        match self.hair_color.as_ref() {
            Some(hex) => {
                if hex.len() != 7 {
                    return false;
                }
                for (i, char) in hex.chars().enumerate() {
                    if i == 0 {
                        if char != '#' {
                            return false;
                        }
                    } else {
                        if !matches!(char, '0'..='9' | 'a'..='f') {
                            return false;
                        }
                    }
                }
                true
            }
            None => false,
        }
    }

    fn validate_eye_color(&self) -> bool {
        match &self.eye_color {
            Some(s) => matches!(
                s.as_str(),
                "amb" | "blu" | "brn" | "grn" | "gry" | "hzl" | "oth"
            ),
            _ => false,
        }
    }

    fn validate_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(s) => {
                if s.len() != 9 {
                    return false;
                }
                for char in s.chars() {
                    if !matches!(char, '0'..='9') {
                        return false;
                    }
                }
                true
            }
            None => false,
        }
    }
}

fn validate_year(s: &Option<String>, min: u16, max: u16) -> bool {
    match s {
        Some(s) => match s.parse::<u16>() {
            Ok(year) => year >= min && year <= max,
            Err(_) => false,
        },
        None => false,
    }
}
