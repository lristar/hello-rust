use std::env;

#[test]
// cargo test test_cmd -- --nocapture lizhuyou gogog.txt
fn test_cmd(){
    let args: Vec<String>=env::args().collect();
      let query = &args[3];
      let filename = &args[4];

      println!("Searching for {}", query);
      println!("In file {}", filename);
}
