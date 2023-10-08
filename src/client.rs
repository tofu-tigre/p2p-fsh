use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{
  AsyncReadExt,
  AsyncWriteExt,
};

use crate::server::{ServerMessage, Received};

pub async fn run_client(v: ServerMessage) -> Result<(), Box<dyn Error>> {
  let mut stream = TcpStream::connect("localhost:8080").await?;
  
  match v {
    ServerMessage::Ping => println!("PING!"),
    ServerMessage::Pong => println!("PONG!"),
  }
  stream.write_u8(v as u8).await?;

  let recv_val;
  match stream.read_u8().await {
      Ok(val) => recv_val = val,
      Err(e) => { return Err(e)? }
  };

  match recv_val.into() {
    Received::Valid(ServerMessage::Pong) => println!("PONG!"),
    Received::Valid(ServerMessage::Ping) => println!("PING!"),
    Received::Invalid(_) => return Err("Invalid message received by client")?,
  }
  Ok(())
}
