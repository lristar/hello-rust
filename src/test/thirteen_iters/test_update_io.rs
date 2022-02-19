use std::env;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[test]
fn test_assert1() {
    let contents = fs::read_to_string("lzyfile.txt".to_string());
    match contents {
        Ok(ct) => {
            let f = search("nobody", &ct);
            println!("hahaha{:#?}", f);
        }
        Err(err) => {
            println!("err is {}", err)
        }
    }
}
// 修改后用迭代器
fn search<'a>(query: &'a str, contest: &'a str) -> Vec<&'a str> {
  let v:Vec<&str>=contest.lines().filter(|x|x.contains(query)).collect();
  v
}
