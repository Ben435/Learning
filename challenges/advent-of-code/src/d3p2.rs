use std::fs::File;
use std::io::prelude::*;
use std::env::args;

const STRATEGIES: &'static [(usize, usize)] = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

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

    let map: Vec<Vec<bool>> = buffer
        .split('\n')
        .map(|s| s.chars().map(|chr| chr == '#').collect::<Vec<bool>>())
        .filter(|v| v.len() > 0)
        .collect();

    // Assume start position empty
    let mut product_of_trees = 1;
    for (x_offset, y_offset) in STRATEGIES {
        let mut num_trees = 0;
        for (i, row) in map.iter().enumerate().skip(1) {
            if i % *y_offset != 0 {
                println!("Continueing: ({}, {}) -> {}", x_offset, y_offset, i);
                continue;
            }
            let cur_x_index = ((i / y_offset) * x_offset) % row.len();
            if *row.get(cur_x_index).unwrap() {
                num_trees += 1;
            }
    
            // Debug logging
            // let tmp_str = row
            //     .iter()
            //     .enumerate()
            //     .map(|(x, &val)| {
            //         if x == cur_x_index {
            //             if val {
            //                 return "X";
            //             } else {
            //                 return "O";
            //             }
            //         } else {
            //             if val {
            //                 return "#";
            //             } else {
            //                 return ".";
            //             }
            //         }
            //     }).collect::<Vec<&str>>().join("");
            // println!("{}", tmp_str);
        }

        println!("Got {} for strategy: ({}, {})", num_trees, x_offset, y_offset);

        if num_trees > 0 {
            product_of_trees *= num_trees;
        }
    }
    

    println!("Product of encountered trees: {}", product_of_trees);

    Ok(())
}
