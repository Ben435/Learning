use std::fs::File;
use std::io::prelude::*;
use std::env::args;

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

    let mut parts = buffer.split("\n").skip(1);
    let options: Vec<(u32, u32)> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|bus_id| bus_id.parse::<u32>().ok())
        .enumerate()
        .flat_map(|(index, bus_id)| match bus_id {
            Some(id) => Some((index as u32, id)),
            None => None,
        })
        .collect();
    
    let first_option = options.iter().map(|(_, bus_id)| bus_id).nth(0).unwrap();

    let mut cur = first_option.clone();
    loop {
        println!("Testing: {}", cur);
        let mut valid = true;
        for (offset, bus_id) in options.iter() {
            if (cur + offset) % bus_id != 0 {
                println!("Failed on {}", bus_id);
                valid = false;
                break;
            }
        }

        if valid {
            println!("Found it!: {}", cur);
            break;
        }

        cur += first_option;
    }

    Ok(())
}
