use std::env;
use std::process;
use minigrep::config::Config;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing cli args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        println!("Application error: {}", e);

        process::exit(1);
    }
    process::exit(0);
}
