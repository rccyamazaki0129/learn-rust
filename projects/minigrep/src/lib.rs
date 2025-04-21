//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

//! #自分のクレート
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

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

/// search
/// 検索クエリとコンテンツを受け取り、クエリを含む行のベクタを返す
/// # Arguments
/// * `query` - 検索クエリ
/// * `contents` - 検索対象のコンテンツ
/// # Returns
/// * `Vec<&str>` - クエリを含む行のベクタ
/// # Examples
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.";
/// let result = search(query, contents);
/// assert_eq!(result, vec!["safe, fast, productive."]);
/// ```
/// # Errors
/// * `std::io::Error` - ファイルの読み込みに失敗した場合
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(
            |line| line.contains(query)
        )
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(
            |line| {
                let query = query.to_lowercase();
                line.to_lowercase()
                    .contains(&query)
            }
        )
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("Did not get a query string")
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("Did not get a file name")
        };
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

    #[test]
    #[allow(non_snake_case)]
    fn コマンド引数からConfigを作成する() {
        // 準備
        let args: Vec<String> = vec![
            String::from("execute.app"),
            String::from("test_query"),
            String::from("test_filename.txt")
        ];

        // 実行
        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
            panic!("Problem parsing arguments. {}", err);
        });
        // 評価
        assert_eq!(
            config,
            Config { 
                query: String::from("test_query"),
                filename: String::from("test_filename.txt"),
                case_sensitive: true
            }
        );
    }
}