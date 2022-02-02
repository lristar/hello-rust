use std::env::{self, VarError};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

fn env_vars(env_name:String) -> Result<String, VarError> {
    let goproxy = env::var(env_name); // 获取环境变量GOPROXY
    match goproxy {
        Ok(v) => Ok(v),
        Err(err) => Err(err),
    }
}

#[test]
fn test_env_vars(){
    let content = env_vars("GOPROXY".to_string());
    // match content {
    //     Ok(c) => {
    //       println!("content is {}",c);
    //     },
    //     Err(err) => {
    //         println!("err is {}",err)
    //     },
    // }
  //  let c =  content.map_or_else(default, f);
  //   println!("c is {}",c)
    let k = 21;

let x : Result<_, &str> = Ok("foo");
assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
println!("--------");
let x : Result<&str, _> = Err("bar");
assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);

}


