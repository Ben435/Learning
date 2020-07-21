use std::fmt;

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
#[allow(unused)]
pub enum LogLevel {
    DEBUG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Logger<'a> {
    prefix: &'a str,
    level: LogLevel,
}

impl <'a> Logger<'a> {
    pub fn new(prefix: &'a str) -> Logger<'a> {
        Logger::new_with_level(prefix, LogLevel::INFO)
    }

    pub fn new_with_level(prefix: &'a str, level: LogLevel) -> Logger<'a> {
        Logger {
            prefix,
            level,
        }
    }

    pub fn log(&mut self, level: LogLevel, msg: &str) {
        if level >= self.level {
            println!("{} {}: {}", level, self.prefix, msg);
        }
    }

    pub fn debug(&mut self, msg: &str) {
        self.log(LogLevel::DEBUG, msg);
    }

    pub fn info(&mut self, msg: &str) {
        self.log(LogLevel::INFO, msg);
    }
}
