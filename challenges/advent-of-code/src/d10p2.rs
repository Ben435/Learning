mod lib;

use std::env::args;
use lib::io_helpers::parse_file_list_of_type;
use std::iter;

fn recursive_remove(numbers: &Vec<u32>, index: usize, input: u32, output: u32) -> u32 {
    let mut found_combos = 1;
    for i in index..numbers.len() {
        let before = if i > 0 { 
            numbers.get(i-1).unwrap()
        } else { 
            &input
        };
        let after = if i < numbers.len()-1 {
            numbers.get(i+1).unwrap()
        } else {
            &output
        };

        // If couldn't remove this, don't.
        if after - before > 3 {
            continue;
        }
        let new_combo = numbers[0..i]
            .iter()
            .chain(numbers[i+1..numbers.len()].iter())
            .map(|n| n.clone())
            .collect::<Vec<u32>>();

        println!("Testing {}:{}:{} {:?}", index, i, numbers.len(), new_combo);

        found_combos += recursive_remove(&new_combo, i, input, output);
    }

    found_combos
}

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

    // let combinations = recursive_remove(&numbers, 0, 0, target);
    let combinations = calculate_jumps(&numbers, 0, target);

    println!("Combinations: {}", combinations);

    Ok(())
}
