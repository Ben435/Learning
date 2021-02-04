use std::fs::File;
use std::io::prelude::*;
use std::env::args;

fn find_invalid_num(feed: &Vec<u64>, preamble_length: usize) -> Option<u64> {
    for i in 0..feed.len() - preamble_length {
        let suspect = feed.get(i + preamble_length).unwrap();

        let mut is_valid = false;
        'outer: for j in 0..preamble_length {
            for k in j+1..preamble_length {
                let num_j = feed.get(i + j).unwrap();
                let num_k = feed.get(i + k).unwrap();
                println!("Checking: {} + {} ?= {}", num_j, num_k, suspect);

                if *num_j + *num_k == *suspect {
                    is_valid = true;
                    break 'outer;
                }
            }
        }

        if !is_valid {
            return Some(*suspect);
        }
    }

    None
}

fn find_contiguous_sum<'a>(feed: &'a Vec<u64>, target: u64) -> Option<&'a [u64]> {
    for start in 0..feed.len()-1 {
        for end in start+1..feed.len() {
            println!("Testing [{}..{}].sum() == {}", start, end, target);
            let r = &feed[start..end];
            let this_sum = r.iter().sum::<u64>();
            if this_sum > target {
                break; // This set is pointless, as already too big
            }
            if this_sum == target {
                return Some(r);
            }
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

    let invalid_num = find_invalid_num(&feed, 25).unwrap();

    let contiguous_range = find_contiguous_sum(&feed, invalid_num).unwrap();

    let min = contiguous_range.iter().min().unwrap();
    let max = contiguous_range.iter().max().unwrap();

    println!("Found invalid_num={}, range={:?}, sum={}", invalid_num, contiguous_range, min + max);

    Ok(())
}
