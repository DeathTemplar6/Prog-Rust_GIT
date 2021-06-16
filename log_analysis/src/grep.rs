use std::fs;
use std::error::Error;
use std::slice::Iter;
use std::env;

//struct for grep
pub struct Config {
    pub query: String,
    pub filesnames: Vec<String>,
	pub case_sensitive: bool,
}

impl Config {
	//init config
    pub fn new(mut args: Iter<'_, String>, files: Vec<String>) -> Result<Config, &'static str> {
        if files.len() < 1{
			return Err("Didn't get a file name");
		}

        let query = match args.next() {
            Some(arg) => arg.clone(),
            None => return Err("Didn't get a query string"),
        };
		
		let filesnames = files.clone();

		//check if there is a third args
		let case_sensitive = match args.next() {
			//if there is check if this args is at true for search or not for search_case_insensitive
            Some(arg) => arg.to_lowercase().contains(&"true"),
			//if there is not check if there is an environment variable for search_case_insensitive
            None => env::var("CASE_INSENSITIVE").is_err(), //if not set will return false
        };

        Ok(Config {
            query,
            filesnames,
            case_sensitive,
        })
    }
}

///handle grep: take a config in entrance and launch search. Print the results
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	for filename in config.filesnames.iter() {
		let contents = fs::read_to_string(filename)?;
		println!("File => {}", filename);

		let results = if config.case_sensitive {
			search(&config.query, &contents)
		} else {
			search_case_insensitive(&config.query, &contents)
		};

		for line in results {
			println!("{}", line);
		}
		println!();
		println!();
	}

    Ok(())
}

/// Search for a query inside a text. And return the line having it
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search for a case insensitive query inside a text. And return the line having it
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
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