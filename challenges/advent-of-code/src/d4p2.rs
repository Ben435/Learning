use std::fs::File;
use std::io::prelude::*;
use std::env::args;

use regex::Regex;

#[derive(Default)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiry_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl <'a> Passport<'a> {
    pub fn validate(&self) -> bool {
        let hair_color: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

        if self.birth_year
            .as_ref()
            .map(|v| v.parse::<u32>().ok())
            .flatten()
            .filter(|&v| 1920 <= v && v <= 2002)
            .is_none() {
            return false;
        }
        
        if self.issue_year
            .as_ref()
            .map(|v| v.parse::<u32>().ok())
            .flatten()
            .filter(|&v| 2010 <= v && v <= 2020)
            .is_none() {
            return false;
        }
        
        if self.expiry_year
            .as_ref()
            .map(|v| v.parse::<u32>().ok())
            .flatten()
            .filter(|&v| 2020 <= v && v <= 2030)
            .is_none() {
            return false;
        }
        
        if self.height
            .as_ref()
            .map(|v| v.split_at(v.len()-2))
            .filter(|(v, unit)| {
                let val = v.parse::<u32>();
                if val.is_err() {
                    return false;
                }
                let val = val.unwrap();
                match unit {
                    &"in" => 59 <= val && val <= 76,
                    &"cm" => 150 <= val && val <= 193,
                    _ => false
                }
            })
            .is_none() {
            return false;
        }
        
        if self.hair_color
            .as_ref()
            .filter(|v| hair_color.is_match(v))
            .is_none() {
            return false;
        }
        
        if self.eye_color
            .as_ref()
            .filter(|v| match *v {
                &"amb" => true,
                &"blu" => true,
                &"brn" => true,
                &"gry" => true,
                &"grn" => true,
                &"hzl" => true,
                &"oth" => true,
                _ => false,
            }).is_none() {
            return false;
        }
        
        if self.passport_id
            .as_ref()
            .filter(|v| v.len() == 9)
            .filter(|v| v.parse::<u32>().is_ok())
            .is_none() {
            return false;
        }
        
        if self.country_id.is_none() {
            // don't care
        }
        true
    }
}

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let mut valid_passports = 0;
    for passport_str in buffer.split("\n\n") {
        let mut passport = Passport::default();
        for attr in passport_str.split_whitespace() {
            let mut attr_iter = attr.split(":");
            let key = attr_iter.next().unwrap();
            let val = attr_iter.next();

            match key {
                "byr" => passport.birth_year = val,
                "iyr" => passport.issue_year = val,
                "eyr" => passport.expiry_year = val,
                "hcl" => passport.hair_color = val,
                "hgt" => passport.height = val,
                "ecl" => passport.eye_color = val,
                "pid" => passport.passport_id = val,
                "cid" => passport.country_id = val,
                _ => eprintln!("Encountered weird code: {}", key),
            };
        }

        if passport.validate() {
            valid_passports += 1;
        }
    }

    println!("Found {} valid passports", valid_passports);

    Ok(())
}
