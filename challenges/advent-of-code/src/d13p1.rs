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

    let mut parts = buffer.split("\n");

    let earliest_time = parts.next().map(|num| num.parse::<u32>().ok()).flatten().unwrap();
    let options = parts.next().unwrap();

    let mut min = (0, std::u32::MAX);
    for bus_id in options
        .split(",")
        .filter(|o| o != &"x")
        .flat_map(|bus_id| bus_id.parse::<u32>().ok()) {
            let next_time = ((earliest_time / bus_id) + 1) * bus_id;
            if next_time < min.1 {
                min = (bus_id, next_time);
            }
    }

    println!("Earliest time: {} vs next bus time: {:?}", earliest_time, min);

    println!("Answer: {}", (min.1 - earliest_time) * min.0);

    Ok(())
}
