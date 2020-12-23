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

    let mut cur_coords: (i32, i32) = (0, 0);
    let mut current_facing_degrees = 0;

    for line in buffer.split("\n").filter(|l| l.len() > 0) {
        let op_code = line.chars().next().unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match op_code {
            'N' => cur_coords = (cur_coords.0, cur_coords.1 + value),
            'S' => cur_coords = (cur_coords.0, cur_coords.1 - value),
            'E' => cur_coords = (cur_coords.0 + value, cur_coords.1),
            'W' => cur_coords = (cur_coords.0 - value, cur_coords.1),
            'R' => current_facing_degrees = (current_facing_degrees + value) % 360,
            'L' => current_facing_degrees = (current_facing_degrees - value) % 360,
            'F' => match current_facing_degrees {
                0 => cur_coords = (cur_coords.0 + value, cur_coords.1),
                90 => cur_coords = (cur_coords.0, cur_coords.1 - value),
                180 => cur_coords = (cur_coords.0 - value, cur_coords.1),
                270 => cur_coords = (cur_coords.0, cur_coords.1 + value),
                _ => {
                    println!("Ignoring forward: D{} -> {}{}", current_facing_degrees, op_code, value)
                }
            }
            _ => {
                println!("Ignoring: {}{}", op_code, value);
            },
        };

        if current_facing_degrees < 0 {
            current_facing_degrees = 360 + current_facing_degrees;
        }

        // println!("{:?} facing {}", cur_coords, current_facing_degrees);
    }

    println!("Now at {:?}", cur_coords);

    Ok(())
}
