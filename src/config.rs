use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_insensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("No argument for query exist"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("No argument for filename exist"),
        };

        let is_case_insensitive = env::var("IS_CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            is_case_insensitive,
        })
    }
}
