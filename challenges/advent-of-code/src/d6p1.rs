use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::HashSet;


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
        let mut questions_answered = HashSet::new();
        for question in group.chars().filter(|c| *c != '\n') {
            questions_answered.insert(question);
        }

        let total_questions_answered = questions_answered.len();

        total += total_questions_answered;
    }

    println!("Total: {}", total);

    Ok(())
}
