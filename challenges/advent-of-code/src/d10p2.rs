mod lib;

use std::env::args;
use lib::io_helpers::parse_file_list_of_type;
use std::iter;

fn calculate_jumps(numbers: &Vec<u32>, input: u32, output: u32) -> u64 {
    let numbers: Vec<u32> = iter::once(input)
        .chain(numbers.iter().map(|n| n.clone()))
        .chain(iter::once(output))
        .collect();
    println!("All numbers: {:?}", numbers);

    let mut jumps_from_index: Vec<u64> = Vec::with_capacity(numbers.len());
    jumps_from_index.resize(numbers.len(), 0);

    for (i, num) in numbers.iter().enumerate().rev().skip(1) {
        println!("Calculating from: {}:{}", i, num);
        let mut forward_jumps_from_here = jumps_from_index.get(i+1).unwrap().clone();
        for j in i+2..numbers.len() {
            let jump_target = numbers.get(j).unwrap();
            println!("Jump from {} to {}? -> {}", num, jump_target, jump_target - num <= 3);

            if jump_target - num <= 3 {
                forward_jumps_from_here += 1 + jumps_from_index.get(j).unwrap();
            } else {
                break;
            }
        }
        jumps_from_index[i] = forward_jumps_from_here;

    }

    println!("Final: {:?}", jumps_from_index);

    1 + jumps_from_index.get(0).unwrap()
}

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();
    let mut numbers: Vec<u32> = parse_file_list_of_type(file_path)?;

    let target = numbers.iter().max().unwrap() + 3;

    numbers.sort();

    println!("Numbers: {:?}", numbers);

    let combinations = calculate_jumps(&numbers, 0, target);

    println!("Combinations: {}", combinations);

    Ok(())
}
