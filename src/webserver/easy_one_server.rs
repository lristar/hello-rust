use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
  // ...
  println!("收到了一些内容")
}

pub fn easy_one_server() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080")?;
  //let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  // accept connections and process them serially
  for stream in listener.incoming() {
      handle_client(stream?);
      //handle_client(stream.unwrap());
  }
  Ok(())
}