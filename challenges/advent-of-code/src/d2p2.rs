mod lib;

use std::env::args;
use lib::io_helpers::parse_file_list_of_type;

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();

    let lines: Vec<String> = parse_file_list_of_type(file_path)?;

    let mut valid_passwords = 0;

    for line in lines.iter().filter(|l| l.len() > 0) {
        let mut components = line.split(' ');
        let range = components.next().unwrap();
        let input = components.next().unwrap();
        let password = components.next().unwrap();

        let mut range_iter = range.split('-');
        let first = range_iter
            .next()
            .map(|s| s.parse::<u32>().ok())
            .flatten()
            .unwrap() as usize;
        let second = range_iter
            .next()
            .map(|s| s.parse::<u32>().ok())
            .flatten()
            .unwrap() as usize;

        let target_val = input
            .strip_suffix(':')
            .unwrap()
            .chars()
            .next()
            .unwrap();

        let seen_first = match password.chars().nth(first-1) {
            Some(chr) => chr == target_val,
            None => false,
        };
        let seen_second = match password.chars().nth(second-1) {
            Some(chr) => chr == target_val,
            None => false,
        };

        if seen_first ^ seen_second {
            valid_passwords += 1
        } else {
            println!("Invalid password! {}", line)
        }
    }

    println!("Valid passwords: {}", valid_passwords);

    Ok(())
}
