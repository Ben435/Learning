use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    LOWER,
    HIGHER,
}

fn binary_narrow(directions: Vec<Direction>, lower_bound: u32, upper_bound: u32) -> Option<u32> {
    let (lower, upper) = directions
        .iter()
        .fold((lower_bound, upper_bound), |(prev_low, prev_high), d| {
            let offset = (prev_high - prev_low) / 2;
            if offset < 1 {
                // last check, little different here.
                return match d {
                    Direction::HIGHER => (prev_high, prev_high),
                    Direction::LOWER => (prev_low, prev_low),
                }
            }
            match d {
                Direction::HIGHER => (prev_high - offset, prev_high),
                Direction::LOWER => (prev_low, prev_low + offset),
            }
        });

    if (upper - lower) > 1 {
        return None;
    }

    return Some(lower);
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

    let mut seating_ids: Vec<u32> = buffer
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.split_at(7))
        .map(|(row_specifiers, col_specifiers)| {
            println!("row, col: ({}, {})", row_specifiers, col_specifiers);

            let row_upper_range = 127;
            let row_lower_range = 0;
            let row_directions: Vec<Direction> = row_specifiers.chars().filter_map(|c| match c {
                'B' => Some(Direction::HIGHER),
                'F' => Some(Direction::LOWER),
                _ => None
            }).collect();

            let row_index = binary_narrow(row_directions, row_lower_range, row_upper_range).unwrap();

            let col_upper_range = 7;
            let col_lower_range = 0;
            let col_directions: Vec<Direction> = col_specifiers.chars().filter_map(|c| match c {
                'R' => Some(Direction::HIGHER),
                'L' => Some(Direction::LOWER),
                _ => None
            }).collect();

            let col_index = binary_narrow(col_directions, col_lower_range, col_upper_range).unwrap();

            // Seating ID
            row_index * 8 + col_index
        })
        .collect();

    let mut seating_id_lookup: HashSet<u32> = HashSet::new();
    seating_ids
        .iter()
        .for_each(|id| {
            seating_id_lookup.insert(*id);
        });

    seating_ids.sort();

    let min: u32 = *seating_ids.iter().min().unwrap();
    let max: u32 = *seating_ids.iter().max().unwrap();

    let mut found_it = 0;
    for seating_id in min..max {
        if seating_ids.contains(&seating_id) {
            continue;
        }
        let prev = seating_id_lookup.contains(&(seating_id - 1));
        let next = seating_id_lookup.contains(&(seating_id + 1));

        if prev && next {
            found_it = seating_id;
        }
    }

    println!("Seating id: {}", found_it);


    Ok(())
}
