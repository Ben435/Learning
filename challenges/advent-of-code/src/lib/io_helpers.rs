use std::fs::File;
use std::io::prelude::*;

pub fn parse_file_list_of_type<T>(file_path: String) -> std::io::Result<Vec<T>> where
    T: std::str::FromStr {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let vals = buffer
        .split('\n')
        .filter_map(|s| s.parse::<T>().ok())
        .collect();

    Ok(vals)
}
