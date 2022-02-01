use std::{fs, collections::btree_map::Values};

// 文件放在项目根目录下
#[test]
fn test_read_file() {
    // --snip--
    println!("In file {}", "hello.txt");

    let contents = fs::read_to_string("lzyfile.txt".to_string())
        .expect("Something went wrong reading the file");
 // 一行一行的走
    for v in contents.lines() {
        println!("一行");
        println!("{}",v)
    }
}

#[test]
fn test_assert(){
  let contents = fs::read_to_string("lzyfil.txt".to_string());
  match contents {
      Ok(ct) => {
        let f = search("nobody", &ct);
        println!("hahaha{:#?}",f);
      },
      Err(err) => {
        println!("err is {}",err)
      },
  }
}

fn search<'a>(query:& 'a str, contest: & 'a str) -> Vec<& 'a str>{
  let mut v = Vec::new();
    for values in contest.lines() {
        if values.contains(query){
          v.push(values);
        }
    }
    v
}