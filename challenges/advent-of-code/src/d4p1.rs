use std::fs::File;
use std::io::prelude::*;
use std::env::args;

#[derive(Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiry_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn validate(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }
        
        if self.issue_year.is_none() {
            return false;
        }
        
        if self.expiry_year.is_none() {
            return false;
        }
        
        if self.height.is_none() {
            return false;
        }
        
        if self.hair_color.is_none() {
            return false;
        }
        
        if self.eye_color.is_none() {
            return false;
        }
        
        if self.passport_id.is_none() {
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
            let val = attr_iter.next().map(|v| String::from(v));

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
