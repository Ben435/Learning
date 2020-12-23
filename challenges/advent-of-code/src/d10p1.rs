mod lib;

use std::env::args;
use lib::io_helpers::parse_file_list_of_type;
use std::iter;
use std::collections::HashMap;

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

    let map: HashMap<u32, u32> = iter::once(&0)
        .chain(numbers.iter())
        .zip(numbers.iter().chain(iter::once(&target)))
        .map(|(prev, next)| next-prev)
        .fold(HashMap::new(), |mut map, diff| {
            match map.get(&diff) {
                Some(val) => map.insert(diff, val+1),
                None => map.insert(diff, 1),
            };

            return map;
        });

    println!("Distribution: {:?}", map);
    let distro_3 = map.get(&3).unwrap();
    let distro_1 = map.get(&1).unwrap();
    println!("Distribution diff: {} * {} = {}", distro_3, distro_1, distro_3 * distro_1);

    Ok(())
}
