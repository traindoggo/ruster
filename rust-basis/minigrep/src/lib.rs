use std::env;
use std::io::prelude::*;
use std::{error::Error, fs::File};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// 'cargo run' -i frog ./sample.txt
// 'cargo run' frog ./sample.txt
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if args.len() < 3 {
            return Err("not enough arguments");
        } else if 4 < args.len() {
            return Err("too much enough arguments");
        }

        let (query, filename) = if args.len() == 4 {
            // -i has high priority than env::CASE_INSENSITIVE
            if args[1] == "-i" {
                case_sensitive = false;
            }

            (args[2].clone(), args[3].clone())
        } else {
            (args[1].clone(), args[2].clone())
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

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

// この中に入っているVec<&str> は search 関数が死んだ瞬間に
// drop されるので 戻り値として使えない
// よってlifetimeを設定して 関数の外でも Vec<&str>が
// 生きているようにしないとだめ
//
// どのくらい生きてほしいかと言うと 引数に渡されているcontents と同じ
// という意味でこんな感じの文法になる
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
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
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
