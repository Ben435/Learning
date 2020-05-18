#[derive(Debug)]
pub struct Config {
    pub filepath: String,
    pub query: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();    // Skip first, as its just the executable file path

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Couldn't find 'query'")
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Couldn't find 'filepath'")
        };
    
        return Ok(Config {
            filepath,
            query
        })
    }
}
