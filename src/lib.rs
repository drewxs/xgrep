use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() < 3 {
            return Err("file path is required");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitve() {
        let query = "Lorem";
        let contents = "\
Lorem ipsum dolor sit amet
consectetur adipiscing elit
Cras vel augue vel magna
Donec ultrices et nisilorem";
        assert_eq!(vec!["Lorem ipsum dolor sit amet"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Lorem";
        let contents = "\
Lorem ipsum dolor sit amet
consectetur adipiscing elit
Cras vel augue vel magna
Donec ultrices et nisilorem";
        assert_eq!(
            vec!["Lorem ipsum dolor sit amet", "Donec ultrices et nisilorem"],
            search_case_insensitive(query, contents)
        );
    }
}
