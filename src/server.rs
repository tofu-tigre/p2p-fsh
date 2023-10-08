use std::error::Error;
use std::net::SocketAddr;
use tokio::{
    net::TcpListener,
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    }
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ServerMessage {
    Ping = 23,
    Pong = 24,
}

pub enum Received {
  Valid(ServerMessage),
  Invalid(u8),
}

impl Into<Received> for u8 {
  fn into(self) -> Received {
      match self {
        23 => Received::Valid(ServerMessage::Ping),
        24 => Received::Valid(ServerMessage::Pong),
        _ => Received::Invalid(self),
        }
    }
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
  let listener = TcpListener::bind("localhost:8080").await?;
  loop {
      spawn_client_listener_task(&listener).await?;
  }
}

async fn spawn_client_listener_task(listener: &TcpListener) ->
    Result<(), Box<dyn Error>> {
    let (socket, addr) = listener.accept().await?;
    println!("Client {addr} connected to server");
    tokio::spawn(async move {
        let rv = ping_pong(socket, addr).await;
        match rv {
          Ok(()) => println!("Client {addr} disconnected"),
          Err(e) => println!("Error: {e}"),
        }
    });
    Ok(())
}

async fn ping_pong(mut socket: tokio::net::TcpStream, _addr: SocketAddr) -> Result<(), Box<dyn Error>> {
  let recv_val;
  match socket.read_u8().await {
      Ok(val) => recv_val = val,
      Err(e) => { return Err(e)? }
  };

  let send_val;
  match recv_val.into() {
    Received::Valid(ServerMessage::Pong) => {
      send_val = ServerMessage::Ping;
    },
    Received::Valid(ServerMessage::Ping) => {
      send_val = ServerMessage::Pong;
    },
    Received::Invalid(_) => return Err("Invalid message received by server")?,
  };

  socket.write_u8(send_val as u8).await?;
  Ok(())
}