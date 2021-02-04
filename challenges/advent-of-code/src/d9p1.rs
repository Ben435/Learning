use std::fs::File;
use std::io::prelude::*;
use std::env::args;

fn find_invalid_num(feed: Vec<u64>, preamble_length: usize) -> Option<u64> {
    for i in 0..feed.len() - preamble_length {
        let suspect = feed.get(i + preamble_length).unwrap();

        let mut is_valid = false;
        for j in 0..preamble_length {
            for k in j+1..preamble_length {
                let num_j = feed.get(i + j).unwrap();
                let num_k = feed.get(i + k).unwrap();
                println!("Checking: {} + {} ?= {}", num_j, num_k, suspect);

                if *num_j + *num_k == *suspect {
                    is_valid = true;
                    break;
                }
            }
            if is_valid {
                break;
            }
        }

        if !is_valid {
            return Some(*suspect);
        }
    }

    None
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

    let feed: Vec<u64> = buffer
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let invalid_num = find_invalid_num(feed, 25);

    println!("Found invalid num: {:?}", invalid_num);

    Ok(())
}
