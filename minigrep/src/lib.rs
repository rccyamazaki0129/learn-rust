use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let results = search(&config.query, &contents);

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

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// テストコード
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn safe_fast_productiveの1行を返す() {
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
    fn Rustの1行を返す() {
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
}