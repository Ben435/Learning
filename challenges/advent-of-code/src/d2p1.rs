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
        let low = range_iter.next().map(|s| s.parse::<u32>().ok()).flatten().unwrap();
        let high = range_iter.next().map(|s| s.parse::<u32>().ok()).flatten().unwrap();

        let target_val = input.strip_suffix(':').unwrap().chars().next().unwrap();

        let mut target_seen = 0;
        for chr in password.chars() {
            if chr == target_val {
                target_seen += 1;
            }
        }

        if low <= target_seen && target_seen <= high {
            valid_passwords += 1
        } else {
            println!("Invalid password! {}", line)
        }
    }

    println!("Valid passwords: {}", valid_passwords);

    Ok(())
}

