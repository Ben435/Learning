use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::HashMap;


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

    let mut total = 0;
    for group in buffer.split("\n\n") {
        let mut questions_answered: HashMap<char, u32> = HashMap::new();
        let num_of_people: u32 = group
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|_| 1)
            .sum();
        for question in group.chars().filter(|c| *c != '\n') {
            match questions_answered.get(&question) {
                Some(prev) => questions_answered.insert(question, prev + 1),
                None => questions_answered.insert(question, 1),
            };
        }

        let total_questions_answered: u32 = questions_answered
            .iter()
            .map(|(k, v)| v)
            .filter(|v| **v >= num_of_people)
            .map(|_| 1)
            .sum();

        total += total_questions_answered;
    }

    println!("Total: {}", total);

    Ok(())
}
