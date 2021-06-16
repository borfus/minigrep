use std::error::Error;
use std::fs;
use std::env;

use clap::ArgMatches;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: ArgMatches) -> Result<Config, &str> {
        let query = match args.value_of("QUERY") {
            Some(q) => q,
            None => panic!("Error obtaining QUERY argument")
        };
        let query = String::from(query);

        let filename = match args.value_of("FILENAME") {
            Some(q) => q,
            None => panic!("Error obtaining FILENAME argument")
        };
        let filename = String::from(filename);

        if let true = args.is_present("INSENSITIVE") {
            env::set_var("CASE_INSENSITIVE", "1");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    println!("{}", query);
    contents.lines()
            .filter(|line| line.to_lowercase().contains(query.as_str()))
            .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

