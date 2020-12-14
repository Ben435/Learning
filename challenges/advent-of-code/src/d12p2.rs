use std::fs::File;
use std::io::prelude::*;
use std::env::args;

fn rotate_point(point: (i32, i32), rotate_by: i32) -> (i32, i32) {
    match rotate_by {
        0 => point,
        90 => (point.1, -point.0),
        180 => (-point.0, -point.1),
        270 => rotate_point(rotate_point(point, 180), 90),
        _ => {
            println!("Ignoring: {:?} rotate_by {}", point, rotate_by);
            point
        }
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

    let mut cur_ship_coords: (i32, i32) = (0, 0);
    let mut cur_waypoint_coords: (i32, i32) = (10, 1);

    for line in buffer.split("\n").filter(|l| l.len() > 0) {
        let op_code = line.chars().next().unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match op_code {
                'N' => cur_waypoint_coords = (cur_waypoint_coords.0, cur_waypoint_coords.1 + value),
            'S' => cur_waypoint_coords = (cur_waypoint_coords.0, cur_waypoint_coords.1 - value),
            'E' => cur_waypoint_coords = (cur_waypoint_coords.0 + value, cur_waypoint_coords.1),
            'W' => cur_waypoint_coords = (cur_waypoint_coords.0 - value, cur_waypoint_coords.1),
            'R' => cur_waypoint_coords = rotate_point(cur_waypoint_coords, value),
            'L' => cur_waypoint_coords = rotate_point(cur_waypoint_coords, 360 - value),
            'F' => cur_ship_coords = (cur_ship_coords.0 + cur_waypoint_coords.0 * value, cur_ship_coords.1 + cur_waypoint_coords.1 * value),
            _ => {
                println!("Ignoring: {}{}", op_code, value);
            },
        };

        println!("{}: {:?} facing {:?}", line, cur_ship_coords, cur_waypoint_coords);
    }

    println!("Now at {:?}", cur_ship_coords);

    Ok(())
}
