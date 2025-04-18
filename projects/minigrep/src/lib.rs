use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

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

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

// テストコード
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn ductを受け取ってsafe_fast_productiveの1行を返す() {
        // 準備
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        // 実行& 評価
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn ustを受け取ってRustの1行を返す() {
        // 準備
        let query = "ust";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        // 実行& 評価
        assert_eq!(
            vec!["Rust:"],
            search(query, contents)
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn rustを受け取ってRustを見つける() {
        // 準備
        let query = "rust";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        // 実行
        // 評価
        assert_eq!(
            vec!["Rust:"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn rEEを受け取ってthreeを見つける() {
        // 準備
        let query = "rEE";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        // 実行
        // 評価
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(query, contents)
        );
    }
}