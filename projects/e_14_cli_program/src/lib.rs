use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //if args.len() < 3 {
        //    return Err("Not enough arguments!");
        //}
        // We need to clone the values because we are borrowing them from the args vector,
        // if we don't clone them, the ownership of the values will be moved to the Config struct
        // which is a violation of the borrowing rules.
        // This operation might be expensive if the values are large.
        //let query = args[1].clone();
        //let file_path = args[2].clone();

        // After refactoring using an iterator.
        args.next(); // Skip the first argument which is the program name.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query not found!"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not found!"),
        };

        // The env::var function returns a Result type, if the variable is not found it returns an Err value.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> is a trait object, it means that the function can return any type that implements the Error trait.
    // dyn is short for dynamic, it means that the type is determined at runtime.
    // The ? operator is used to propagate errors up the call stack.
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The lifetime 'a is the lifetime of the reference to the contents parameter.
    // The return value of this function is a vector of string slices that are references to the contents parameter.
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\\\nRust:\nsafe, fast, productive.\nPick three.\n";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\\\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.\n";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
