mod lib;

use std::env::args;
use lib::io_helpers::parse_file_list_of_type;

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Usage: day-1 <input-file.txt>");
        return Ok(());
    }
    let file_path = arg.unwrap();

    let numbers: Vec<u32> = parse_file_list_of_type(file_path)?;

    for x in numbers.iter() {
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == 2020 {
                    println!("Found: {} * {} * {} = {}", x, y, z, x*y*z)
                }
            }
        }
    }

    Ok(())
}
