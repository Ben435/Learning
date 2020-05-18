use std::fs;
use std::error::Error;
pub mod config;

pub fn run(conf: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.filepath)?;

    for line in search(&conf.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| { line.contains(query) })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const DUMMY_CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn no_results() {
        let query = "carrot";
        
        assert_eq!(Vec::<String>::new(), search(query, DUMMY_CONTENTS))
    }

    #[test]
    fn one_result() {
        let query = "duct";
        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, DUMMY_CONTENTS)
        )
    }

    #[test]
    fn multiple_result() {
        let query = "i";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."], 
            search(query, DUMMY_CONTENTS)
        )
    }
}
