use crate::Config;
use crate::SearchHandler;
use std::fs;

pub struct Runner;

impl Runner {
    pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(&config.filename)?;
        let search_result = if config.is_case_insensitive {
            SearchHandler::search_case_insensitive(&config.query, &contents)
        } else {
            SearchHandler::search(&config.query, &contents)
        };

        for line in search_result {
            println!("{}", line);
        }

        Ok(())
    }
}
